DIR ?= ~/.plotline
PLUGINS_DIR ?= ${DIR}/plugins
WASM_TARGET = wasm32-wasi

all: dist install clean

dist:
	@rustup target list | grep ${WASM_TARGET}
	# @cargo build --package event_is_not_experienced_more_than_once --target ${WASM_TARGET} --release
	# @cargo build --package experience_follows_previous --target ${WASM_TARGET} --release
	@cargo build --package experience_is_not_simultaneous --target ${WASM_TARGET} --release
	# @cargo build --package experience_precedes_next --target ${WASM_TARGET} --release

	@mkdir -p /tmp/.plotline/plugins

	@cp -f ../LICENSE ../target/${WASM_TARGET}/release/*.wasm /tmp/.plotline/plugins
	@tar -czf plugins.tar.gz -C /tmp/.plotline/plugins .
	
	@rm -rf /tmp/.plotline/plugins
	@rm -rf ../target/${WASM_TARGET}/release/*.wasm

install:
	@mkdir -p ${PLUGINS_DIR}
	@tar -xzf plugins.tar.gz -C ${PLUGINS_DIR}

uninstall:
	@rm -rf ${PLUGINS_DIR}/*

clean:
	@rm -rf /tmp/.plotline/plugins
	@rm -rf plugins.tar.gz