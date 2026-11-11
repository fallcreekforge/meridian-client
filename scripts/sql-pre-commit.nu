# Compare file contents so existing unstaged edits do not hide formatter changes.
let files = (glob crates/**/*.sql | sort)
let before = ($files | each {|file| open --raw $file | hash sha256 })

let result = (sqlfluff fix crates | complete)
print --no-newline $result.stdout
print --stderr --no-newline $result.stderr

let changed = ($files | zip $before | where {|entry|
    (open --raw $entry.0 | hash sha256) != $entry.1
} | each {|entry| $entry.0 })

if ($changed | is-not-empty) {
    print --stderr "SQLFluff changed SQL files:"
    $changed | each {|file| print --stderr $file } | ignore
    print --stderr "Review and stage the changes, then retry the commit."
    exit 1
}

exit $result.exit_code
