**Disclaimer:** This project is very much WIP as of 2024-05-04, use at your own discretion.

# Process Notify (pntfy)

## Is this you?

Have you ever started up a long-running process, only to come back hours later to find it failed within seconds of leaving it unattended? Would you want to know if a process failed, or succeeded, earlier than whenever you remember to check on it? If you answered yes to either of those questions, `pntfy` might be helpful to you.

## About

`pntfy` (pronounced 'puntify') is a tool that notifies you of the state of a long-running process, allowing you to use your time more effectively and avoid unpleasant surprises. Notifications are provided by any instance (self-hosted or official) of [ntfy.sh](https://ntfy.sh). `pntfy` is highly configurable, but simple to use for common use-cases, take a look at the example below to see it in action.

## Example

All you need to do to get started is pass the command/program you wish to monitor to `pntfy`.

``` txt
$ pntfy ./test.sh
```

`pntfy` will generate a ntfy URL, which when followed will take you straight to the ntfy dashboard.

![Terminal output](/assets/2024-05-04-example-1.png)

`pntfy` will issue notifications for errors, as well as the exit status of the monitored program.

![Ntfy.sh dashboard](/assets/2024-05-04-example-2.png)

## Usage

```
A tool for notifying when a command fails or succeeds

Usage: pntfy [OPTIONS] <COMMAND>

Arguments:
  <COMMAND>  The command to monitor

Options:
  -t, --topic <TOPIC>              Use a custom notification topic
      --ntfy-server <NTFY_SERVER>  The ntfy server url [default: http://ntfy.sh]
  -c, --no-cache                   Request that the ntfy server disables caching messages
  -f, --no-firebase                Request that the ntfy server disables forwarding messages to Firebase
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```
