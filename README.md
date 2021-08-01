# Fallout Terminal Sound

Small utility to make your daily terminal usage more fun

# Usage

```sh
# if program input is 0 or none - play success sound, otherwise fail one
fo_term_sound $?
```

## zsh integragion

To get a sound after each command you type in zsh finished edit your `~/.zshrc`:

```sh
ok_or_fail() {
    fo_term_sound $?
}

autoload -Uz add-zsh-hook
add-zsh-hook precmd ok_or_fail
```

# TODO

## Daemonize

Make it a daemon that will use named pipe or unix socket.
Zsh hook will send last cmd status into it and daemon will play the sound.

Why?
- Not to exhaust your pids for playing sub 200 ms sound after each shell command typed!
- Daemon can be stopped / continued, so that user can turn these sounds on/off.

```
pkill -SIGSTOP fo_term_sound
pkill -SIGCONT fo_term_sound
```

## Symphonize

Make it via `symphonia-play` since current implementation is kinda broken, needs thread::sleep otherwise terminates early.
