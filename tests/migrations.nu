def fixture [root: path name: string] {
    let directory = ($root | path join $name)
    mkdir ($directory | path join "crates/meridian-store/migrations")
    "-- Synthetic declarative SQLite schema.\n" | save ($directory | path join "crates/meridian-store/schema.sql")
    ".local/\n" | save ($directory | path join ".gitignore")
    git init --quiet --initial-branch main $directory | ignore
    git -C $directory config user.name "Synthetic migration test" | ignore
    git -C $directory config user.email "migration-test@example.invalid" | ignore
    git -C $directory config commit.gpgsign false | ignore
    git -C $directory config core.hooksPath ($directory | path join ".git/hooks") | ignore
    git -C $directory add . | ignore
    git -C $directory commit --quiet -m "Synthetic base" | ignore
    git -C $directory remote add origin $directory | ignore
    git -C $directory fetch --quiet origin main:refs/remotes/origin/main | ignore
    git -C $directory switch --quiet -c feature | ignore
    $directory
}

def expect-pass [result: record label: string] {
    if $result.exit_code != 0 {
        error make {msg: $"($label) failed:\n($result.stdout)\n($result.stderr)"}
    }
    print $"PASS: ($label)"
}

def expect-failure [result: record label: string message: string = ""] {
    if $result.exit_code == 0 or not ($result.stderr | str contains $message) {
        error make {msg: $"($label) was not rejected as expected:\n($result.stdout)\n($result.stderr)"}
    }
    print $"PASS: ($label)"
}

def main [migration_script: path] {
    let script = ($migration_script | path expand)
    let original = $env.PWD
    let root = (mktemp -d)
    let schema = "crates/meridian-store/schema.sql"
    let migrations = "crates/meridian-store/migrations"
    try {
        cd (fixture $root "generation")
        expect-pass (nu $script check | complete) "code-only change with a comment-only schema and no history"
        expect-pass (nu $script generate no_change | complete) "skip migration generation for an unchanged schema"
        mkdir .local
        sqlite3 .local/meridian.db "CREATE TABLE unrelated_local_drift (id INTEGER);" | ignore
        "-- Synthetic declarative SQLite schema.\nCREATE TABLE games (id INTEGER PRIMARY KEY, name TEXT NOT NULL);" | save --force $schema
        expect-failure (nu $script check | complete) "schema change without migration" "PR migrations do not produce"
        expect-pass (nu $script generate create_games | complete) "generate from main despite local database drift"
        mut generated = (glob $"($migrations)/*.sql" | first)
        if (open --raw $generated | str contains "unrelated_local_drift") {
            error make {msg: "The generated migration included local database drift."}
        }
        expect-pass (nu $script check | complete) "replay generated migration"
        expect-failure (nu $script generate duplicate | complete) "refuse a second branch migration" "branch migration already exists"
        "-- synthetic extra migration\n" | save ($migrations | path join "99999999999999_extra.sql")
        expect-failure (nu $script check | complete) "reject two new migrations" "at most one"
        rm ($migrations | path join "99999999999999_extra.sql")
        let migration_sql = (open --raw $generated)
        $"($migration_sql)\n-- synthetic checksum change\n" | save --force $generated
        expect-failure (nu $script check | complete) "reject stale migration checksum"
        $migration_sql | save --force $generated

        let published = ($migrations | path join "20200101000000_create_games.sql")
        mv $generated $published
        $generated = $published
        atlas migrate hash --dir $"file://($migrations)"
        git add .
        git commit --quiet -m "Publish synthetic migration"
        git branch --force main HEAD
        git fetch --quiet origin main:refs/remotes/origin/main
        expect-pass (nu $script check | complete) "reconstruct a nonempty main baseline"
        $"($migration_sql)\n-- synthetic history edit\n" | save --force $generated
        expect-failure (nu $script check | complete) "reject edits to merged history" "cannot be changed"
        $migration_sql | save --force $generated
        rm $generated
        expect-failure (nu $script check | complete) "reject deletions from merged history" "cannot be deleted"
        $migration_sql | save $generated
        "-- synthetic out-of-order migration\n" | save ($migrations | path join "00000000000000_before.sql")
        expect-failure (nu $script check | complete) "reject out-of-order migration" "must sort after"
        rm ($migrations | path join "00000000000000_before.sql")
        "-- synthetic duplicate version\n" | save ($migrations | path join "20200101000000_zz_duplicate.sql")
        expect-failure (nu $script check | complete) "reject duplicate migration version" "must be greater"
        rm ($migrations | path join "20200101000000_zz_duplicate.sql")
        "CREATE TABLE games (id INTEGER PRIMARY KEY, name TEXT NOT NULL, summary TEXT);" | save --force $schema
        expect-pass (nu $script generate add_summary | complete) "generate an upgrade from nonempty main"
        expect-pass (nu $script check | complete) "replay an upgrade from nonempty main"

        cd (fixture $root "invalid-sql")
        "INVALID SYNTHETIC SQL;" | save ($migrations | path join "20200101000000_invalid.sql")
        atlas migrate hash --dir $"file://($migrations)"
        expect-failure (nu $script check | complete) "reject invalid migration SQL"

        cd (fixture $root "views")
        "CREATE VIEW selected AS SELECT 1 AS id;" | save --force $schema
        expect-failure (nu $script generate create_view | complete) "reject silently omitted view generation" "cannot produce"
        "CREATE VIEW selected AS SELECT 1 AS id;" | save ($migrations | path join "20200101000000_create_view.sql")
        atlas migrate hash --dir $"file://($migrations)"
        expect-pass (nu $script check | complete) "verify an explicitly authored view migration"

        cd (fixture $root "base-drift")
        "CREATE TABLE missing_history (id INTEGER);" | save --force $schema
        git add .
        git commit --quiet -m "Synthetic inconsistent main"
        git branch --force main HEAD
        git fetch --quiet origin main:refs/remotes/origin/main
        expect-failure (nu $script check | complete) "reject schema drift in main" "Base migrations do not match"

        cd (fixture $root "stale-main")
        git switch --quiet main
        "Synthetic upstream change" | save "upstream.txt"
        git add .
        git commit --quiet -m "Advance synthetic main"
        git switch --quiet feature
        expect-failure (nu $script generate stale | complete) "refresh and reject outdated main" "Update this branch"
    } catch {|failure|
        cd $original
        rm --recursive --force $root
        error make {msg: $failure.msg}
    }
    cd $original
    rm --recursive --force $root
}
