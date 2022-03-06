# nes emulator
based on https://bugzmanov.github.io/nes_ebook/

## run tests
```
cargo test
```

## run "UI"
- some ld error
```
  = note: /usr/bin/ld: cannot find -lSDL2
          collect2: error: ld returned 1 exit status
```
- solution:
```
sudo apt-get install libsdl2-dev    # worked
```

## run
```
cargo run
```

## debug nes-rom
- list all non-empty hex-rows
```
xxd -i snake.nes | grep -v "0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00"
```