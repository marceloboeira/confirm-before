#!/bin/sh

printf "Running 'confirm-before' tests: \n"

printf "  when a 'y' is given it runs the command - "
echo "y\n" | confirm-before echo result | grep -q result; if [ $? != 0 ]; then echo "FAIL"; exit 1; else echo "PASS"; fi

printf "  when a 'n' is given it does not run the command - "
echo "n\n" | confirm-before echo result | grep -q result; if [ $? != 0 ]; then echo "PASS"; else echo "FAIL"; exit 1; fi

printf "  when a 'p' is given it does not run the command - "
echo "p\n" | confirm-before echo result | grep -q result; if [ $? != 0 ]; then echo "PASS"; else echo "FAIL"; exit 1; fi

printf "  when a complex command is given - "
echo "y\n" | confirm-before cat ./tests/resource.txt | grep -q content; if [ $? != 0 ]; then echo "FAIL"; exit 1; else echo "PASS"; fi