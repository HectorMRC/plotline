DIR ?= ~/.plotline
PLUGINS_DIR ?= ${DIR}/plugins
WASM_TARGET = wasm32-wasi

all: dist install clean

dist:
	@rustup target list | grep ${WASM_TARGET}
	
	@for plugin in */; do  \
		cargo build --package $${plugin///} --target ${WASM_TARGET} --release; \
    done

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