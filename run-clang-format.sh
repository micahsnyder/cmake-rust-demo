#!/bin/bash

clang-format -style='{ Language: Cpp, UseTab: Never, IndentWidth: 4, AlignTrailingComments: true, AlignConsecutiveAssignments: true, AlignAfterOpenBracket: true, AlignEscapedNewlines: Left, AlignOperands: true, AllowShortFunctionsOnASingleLine: Empty, AllowShortIfStatementsOnASingleLine: true, AllowShortLoopsOnASingleLine: true, BreakBeforeBraces: Linux, BreakBeforeTernaryOperators: true, ColumnLimit: 0, FixNamespaceComments: true, SortIncludes: false, MaxEmptyLinesToKeep: 1, SpaceBeforeParens: ControlStatements, IndentCaseLabels: true, DerivePointerAlignment: true }' -dump-config > .clang-format

find ./ -iname *.h -o -iname *.c | xargs clang-format -i -verbose
