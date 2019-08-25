#!/bin/bash
set +e

if [ "$1" = "--check" ]; then
    diff -q <(cargo readme | $0 --filter) README.md || (
       printf '\033[1;31mREADME needs to be re-generated.\n'
       printf 'Run `cargo readme > README.md`\033[0m\n'
       exit 1
    )
elif [ "$1" = "--filter" ]; then
    sed -E '/\[`.*`\]: .*(struct|enum|trait|type|fn|index)\./d' |
    sed -e 's/\[`\([^]]*\)`\]/`\1`/g'
else
    cargo readme | $0 --filter > README.md
fi

