# [Args]
# OPT = -f以外のOPT
#  (ex.) make debug OPT="build --no-cache"

LOG_LEVEL := debug
export RUST_LOG=$(LOG_LEVEL)
# https://qiita.com/syui/items/e071ba72ea82d583e380
#PREFIX := $(HOME)/.cargo

ifeq ($(OS),Windows_NT)
 	SLASH = \\
else
	SLASH = /
endif

DEBUG_TARGET = target$(SLASH)debug$(SLASH)sound-compose.exe
DEBUG_YAML_FILE = make_debug-sound-compose.yml

ifeq ($(OPT),)
 	OPT = "gui"
endif

ifeq ($(OS),Windows_NT)
	# TODO: SDL2がうまく動かないため暫定対応中
	DEBUG_COMMAND = set VERBOSE=1 && cargo build --verbose && copy /Y make\dll\SDL2\* target\debug\ && cmd.exe /C $(DEBUG_TARGET) -f $(DEBUG_YAML_FILE) ${OPT}
else
	# TODO: Linux環境でのmake debug動作確認が未
	DEBUG_COMMAND = cargo build && $(DEBUG_TARGET) -f $(DEBUG_YAML_FILE) ${OPT}
endif

debug:
	$(DEBUG_COMMAND)

# https://qiita.com/syui/items/e071ba72ea82d583e380
#run:
#    cargo run $(APP_ARGS)
#
#test:
#    cargo test
#
#check:
#    cargo check $(OPTION)
#
#install:
#    cargo install --force --root $(PREFIX) --path .