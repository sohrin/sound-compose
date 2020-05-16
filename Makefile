LOG_LEVEL := debug
export RUST_LOG=$(LOG_LEVEL)
# https://qiita.com/syui/items/e071ba72ea82d583e380
#PREFIX := $(HOME)/.cargo

ifeq ($(OS),Windows_NT)
	WIN_FLG = true
 	SLASH = \\
else
	WIN_FLG = false
	SLASH = /
endif

DEBUG_TARGET = target$(SLASH)debug$(SLASH)sound-compose.exe
DEBUG_YAML_FILE = make_debug-sound-compose.yml

ifeq (WIN_FLG,true)
	DEBUG_COMMAND = cargo build && cmd.exe /C $(DEBUG_TARGET) -f $(DEBUG_YAML_FILE) build --no-cache
else
	# TODO: Linux環境でのmake debug動作確認が未
	DEBUG_COMMAND = cargo build && $(DEBUG_TARGET) -f $(DEBUG_YAML_FILE) build --no-cache
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