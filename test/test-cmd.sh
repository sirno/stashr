#!/usr/bin/env bash
#
# test.sh: run some stashr commands


function create_directory {
    mkdir subdir
    touch a
    touch subdir/b
}

function test_invocation {
    echo "Testing invocations in $(pwd) ..."

    stashr a
    if [[ -f a ]]; then
        echo "Could not push a file"
    fi

    stashr
    if [[ ! -f a ]]; then
        echo "Could not pop a file"
    fi

    stashr subdir
    if [[ -d subdir ]]; then
        echo "Could not push a dir"
    fi

    stashr
    if [[ ! -d subdir ]]; then
        echo "Could not pop a dir"
    fi

    stashr a subdir
    if [ `ls -1 2> /dev/null | wc -l` -gt 0 ]; then
        echo "Could not push all"
    fi

    stashr
    if [ `ls -1 2> /dev/null | wc -l` -lt 2 ]; then
        echo "Could not pop all"
    fi
}

function test_errors {
    echo "Testing errors in $(pwd) ..."

    msg=`stashr file`
    if [[ $msg != "stashr: file: No such file or directory" ]]; then
        echo "Incorrect error message: $msg"
    fi

    msg=`stashr`
    if [[ $msg != "stashr: \"default\": Stash is empty" ]]; then
        echo "Incorrect error message: $msg"
    fi
}

echo "Running stashr tests ..."
echo "--- --- ---"

# test in local file system
mkdir /usr/test_dir
cd /usr/test_dir

create_directory
test_invocation
test_errors

echo "--- --- ---"

# test in mounted file system
cd /mount/vdrive

create_directory
test_invocation
test_errors

echo "--- --- ---"

echo "Finished..."

