A simple command repeater.

## Usage

`groundhog -c 'echo {}' -t 10`

Will output something like this:

```
1
2
3
4
5
6
7
8
9
10
```

## Why?

You can achieve the basic functionality with a simple `for` loop in any shell or using `seq 10 | xargs -l1 -- sh -c 'echo $1'`.
What this tool (will) provide is a better overview about the runs, especially if the status of the command changes.

Personally I use it for rerunning test suites many times to then detect flaky tests.
