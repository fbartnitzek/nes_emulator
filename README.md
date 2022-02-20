# nes emulator
based on https://bugzmanov.github.io/nes_ebook/

## run tests
```
cargo test
```

## run
- some ld error
```
  = note: /usr/bin/ld: cannot find -lSDL2
          collect2: error: ld returned 1 exit status
```
- solution:
```
sudo apt-get install libsdl2-dev    # worked
```