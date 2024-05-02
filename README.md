# Process Notify (pntfy)

## Is this you?

Have you ever started up a long-running process, only to come back hours later to find it failed within seconds of leaving it unattended? Would you want to know if a process failed, or succeeded, earlier than whenever you remember to check on it? If you answered yes to either of those questions, `pntfy` might be an answer for you.

## About

`pntfy` (pronounced 'puntify') is a tool that notifies you of the state of a long-running process, allowing you to use your time more effectively and avoid unpleasant surprises. Notifications are provided by any instance (self-hosted or official) of [ntfy.sh](https://ntfy.sh). `pntfy` is highly configurable, but simple to use for common use-cases, take a look at the example below to see it in action.

## Example

``` sh
pntfy ./long-running-process.sh

█████████████████████████████████████
█████████████████████████████████████
████ ▄▄▄▄▄ █▀ █▀▀█▄▀██▀▄██ ▄▄▄▄▄ ████
████ █   █ █▀ ▄ ▀▀▄▀ ▀ █▀█ █   █ ████
████ █▄▄▄█ █▀█ █ ▄▄▀▀▄▄▄▀█ █▄▄▄█ ████
████▄▄▄▄▄▄▄█▄█▄█ █ ▀▄█▄█▄█▄▄▄▄▄▄▄████
████▄  ▄▄█▄▄  ▄█  ██▄▄▀▀▀▄▀ ▀▄▀▄▀████
████▄█▄█▄█▄██ ▀█ █   █▀▀▄▄▀▀█▄█▀▀████
█████▄ ▄▄ ▄▀█ ▀█ ▄▀██▄▀▀▄▀▀█▀▄ ▀▀████
█████ ██▄█▄▀█ ▀ █▄█▄█▄▀▀█ ▄▄▄█▄▀█████
████ ██ ▄▄▄ ▄▄ ▄▄▄▀▄▄▄▄▀▀ ▀ ▀▀ █▀████
████ █▀█▄▄▄█ ▄▀█ █▀▄▄▄██▀█ ▀▀▄▄▀▀████
████▄█▄█▄█▄▄▀██ ▄▄▄▄▀▄▄█ ▄▄▄ ▀ ▀ ████
████ ▄▄▄▄▄ █▄██▄▀▄█ ▄▄▄  █▄█  ▄█▀████
████ █   █ █ ██▀ ▄▄▄▀  ▀▄▄▄▄▄▀   ████
████ █▄▄▄█ █ ▄██ ▄▄ ▀██ ▀ ▄▄ ▀▄ █████
████▄▄▄▄▄▄▄█▄▄▄████▄▄▄▄████▄▄▄▄██████
█████████████████████████████████████
█████████████████████████████████████

Scan or click (https://ntfy.sh/a5db3a00-df73-4ac2-9619-21bf48831800) to subscribe to this process's topic.

─( Process Output )────────────────────────────────────────────────────────────────────────────────────────
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec sodales, nunc sed semper dignissim, dolor 
erat fermentum ex, vitae ultricies ligula lorem ac ante. Cras at lectus sed ligula laoreet rhoncus id at 
nisi. Vivamus facilisis quam feugiat risus consectetur, ut luctus ligula cursus. Nam accumsan ipsum vel 
erat fringilla consequat. Suspendisse vel ante vitae libero porta volutpat ut eget tellus. Quisque lacinia 
hendrerit ullamcorper. Pellentesque lacinia iaculis velit, placerat gravida felis molestie ac. In sed 
scelerisque diam. Praesent sagittis turpis id leo blandit aliquam. Aliquam at dui dapibus, congue ex

```

Some time later...

```
─( Process Output )────────────────────────────────────────────────────────────────────────────────────────
ut luctus ligula cursus. Nam accumsan ipsum vel erat fringilla consequat. Fin.

Error: Ran out of Lorem Ipsum!
[pntfy: Exit 1 received, notifying]
```

Any subscribed devices will receive a notification announcing the process failure.

## Usage

> TBD
