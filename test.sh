#! /usr/bin/env sh

if [ "$1" == "fail" ]; then
    echo "This will run for a bit and then fail."
    sleep 2;
    echo "Oh noez, a fail!" >&2
    exit 1
fi

if [ "$1" == "input" ]; then
    echo "This mode is for testing a process that asks for input."
    echo "Please enter a number (0 to fail): "
    read -r input
    if [ "$input" == "0" ]; then
        echo "Error: Zero" >&2
        exit 1
    fi
    echo "Got $input, kthxbai!"
    exit 0
fi

for i in `seq 1 5`; do
    if [ $i == 3 ]; then
        echo "Error: 3 is no good" >&2
    else
        echo "Important output: $i"
    fi
    sleep 1;
done

exit 0
