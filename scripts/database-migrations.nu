const schema = "crates/meridian-store/schema.sql"
const migrations = "crates/meridian-store/migrations"

def --wrapped checked [command: string ...arguments: string] {
    let result = (^$command ...$arguments | complete)
    if $result.exit_code != 0 {
        error make {msg: $"($command) failed:\n($result.stderr)\n($result.stdout)"}
    }
    $result.stdout
}

def sql-files [directory: path] {
    if not ($directory | path exists) { return [] }
    ls -a $directory | where type == file | get name
        | where {|file| ($file | path parse | get extension) == "sql" }
        | each {|file| $file | path basename } | sort
}

def file-url [file: path] {
    $"file://($file | path expand | url encode)"
}

def database-url [file: path] {
    $"sqlite://file:($file | path expand | url encode)"
}

def at-revision [revision: string file: string] {
    checked git show $"($revision):($file)"
}

def with-scratch [operation: closure] {
    let scratch = (checked mktemp -d | str trim)
    try {
        do $operation $scratch
    } catch {|failure|
        rm --recursive --force $scratch
        error make {msg: $failure.msg}
    }
    rm --recursive --force $scratch
}

def assert-schema [database: path desired: path message: string] {
    let difference = (checked atlas schema diff --from (database-url $database)
        --to (file-url $desired) --dev-url "sqlite://dev?mode=memory"
        --exclude atlas_schema_revisions | str trim)
    if $difference not-in ["", "Schemas are synced, no changes to be made."] {
        error make {msg: $"($message)\n($difference)"}
    }
    # Compare objects omitted by the schema comparison engine using SQLite itself.
    let query = "SELECT type, name, sql FROM sqlite_schema WHERE type IN ('view', 'trigger') ORDER BY type, name;"
    let expected = (checked sqlite3 -json :memory: $"(open --raw $desired)\n($query)" | str trim)
    let actual = (checked sqlite3 -json $database $query | str trim)
    if $actual != $expected {
        error make {msg: $"($message) Views and triggers must match schema.sql exactly; supply explicit migration SQL for these objects."}
    }
}

def baseline [revision: string scratch: path] {
    let directory = ($scratch | path join migrations)
    mkdir $directory
    let tracked = (checked git ls-tree -r --name-only $revision -- $migrations | lines)
    for file in $tracked {
        let name = ($file | str replace $"($migrations)/" "")
        if ($name | path dirname) != "" {
            error make {msg: "Migration files must be directly inside the migrations directory."}
        }
        if ($name | str ends-with ".sql") or $name == "atlas.sum" {
            at-revision $revision $file | save ($directory | path join $name)
        }
    }
    let desired = ($scratch | path join "base-schema.sql")
    let has_schema = (checked git ls-tree --name-only $revision -- $schema | str trim) != ""
    if $has_schema { at-revision $revision $schema | save $desired } else { "" | save $desired }
    let database = ($scratch | path join "base.db")
    checked sqlite3 $database "PRAGMA user_version = 0;" | ignore
    let names = (sql-files $directory)
    if not ($names | is-empty) {
        checked atlas migrate apply --dir (file-url $directory) --url (database-url $database) | ignore
    }
    assert-schema $database $desired "Base migrations do not match the base schema.sql."
    {directory: $directory, database: $database, names: $names}
}

def branch-files [base: record] {
    if ($migrations | path exists) {
        if not (ls -a $migrations | where type != file | is-empty) {
            error make {msg: "Migration files must be regular files directly inside the migrations directory."}
        }
    }
    let names = (sql-files $migrations)
    for name in $base.names {
        let current = ($migrations | path join $name)
        if not ($current | path exists) {
            error make {msg: $"Merged migrations cannot be deleted: ($name)"}
        }
        if (open --raw $current) != (open --raw ($base.directory | path join $name)) {
            error make {msg: $"Merged migrations cannot be changed: ($name)"}
        }
    }
    let added = ($names | where {|name| $name not-in $base.names })
    if ($added | length) > 1 {
        error make {msg: "A PR may add at most one SQL migration; consolidate the unmerged migrations."}
    }
    if ($names | take ($base.names | length)) != $base.names {
        error make {msg: "New migrations must sort after the merged migration history."}
    }
    if not ($added | is-empty) {
        let name = ($added | first)
        if $name !~ '^[0-9]{14}_[a-z][a-z0-9_]*\.sql$' {
            error make {msg: "Use a timestamped migration filename: <14-digit-version>_<name>.sql."}
        }
        let version = ($name | split row "_" | first)
        if not ($base.names | is-empty) {
            let previous = ($base.names | last | split row "_" | first)
            if $version <= $previous {
                error make {msg: "New migration versions must be greater than merged versions."}
            }
        }
    }
    $added
}

def resolve-base [base: string] {
    checked git rev-parse --verify --end-of-options $"($base)^{commit}" | str trim
}

# Refresh main and generate a single migration without using the developer's database.
def "main generate" [name: string] {
    if $name !~ '^[a-z][a-z0-9_]*$' {
        error make {msg: "Use a migration name containing lowercase letters, digits, and underscores."}
    }
    checked git fetch origin "+refs/heads/main:refs/remotes/origin/main" | ignore
    let revision = (resolve-base "origin/main")
    let ancestry = (git merge-base --is-ancestor $revision HEAD | complete)
    if $ancestry.exit_code != 0 {
        error make {msg: "Update this branch with origin/main before generating its migration."}
    }
    with-scratch {|scratch|
        let base = (baseline $revision $scratch)
        if not (branch-files $base | is-empty) {
            error make {msg: "A branch migration already exists. Preserve any hand-written SQL before removing it and regenerating."}
        }
        (checked atlas migrate diff $name --dir (file-url $base.directory)
            --to (file-url $schema) --dev-url "sqlite://dev?mode=memory") | ignore
        let added = (sql-files $base.directory | where {|file| $file not-in $base.names })
        if ($added | length) > 1 {
            error make {msg: "The generator produced more than one SQL migration."}
        }
        if ($added | is-empty) {
            assert-schema $base.database $schema "The generator cannot produce the requested schema change."
            print "The schema matches main; no migration is needed."
        } else {
            let generated = ($added | first)
            if not ($base.names | is-empty) {
                let previous = ($base.names | last | split row "_" | first)
                let version = ($generated | split row "_" | first)
                if $version <= $previous {
                    error make {msg: "The generated version must be newer than main's last migration. Check the clock and retry."}
                }
            }
            (checked atlas migrate apply --dir (file-url $base.directory)
                --url (database-url $base.database)) | ignore
            assert-schema $base.database $schema "Generated migration does not produce schema.sql."
            mkdir $migrations
            cp ($base.directory | path join $generated) ($migrations | path join $generated)
            checked atlas migrate hash --dir (file-url $migrations) | ignore
            print $"Created ($migrations)/($generated) and updated atlas.sum."
        }
    }
}

# Check the PR delta and replay it on an isolated database reconstructed from its base.
def "main check" [base: string = "origin/main"] {
    let revision = (resolve-base $base)
    with-scratch {|scratch|
        let baseline = (baseline $revision $scratch)
        branch-files $baseline | ignore
        if not (sql-files $migrations | is-empty) {
            checked atlas migrate validate --dir (file-url $migrations) | ignore
            (checked atlas migrate apply --dir (file-url $migrations)
                --url (database-url $baseline.database)) | ignore
        }
        assert-schema $baseline.database $schema "PR migrations do not produce schema.sql."
    }
    print "Migration policy and schema replay passed."
}

def main [] { help main }
