DIR ?= ~/.plotline
PLUGINS_DIR ?= ${DIR}/plugins

all: dist install clean

dist:
	@rustup target list | grep wasm32-wasi
	@cargo build --package experience_is_not_simultaneous --target wasm32-wasi --release

	@mkdir -p /tmp/.plotline/plugins

	@cp -f ../LICENSE ../target/wasm32-wasi/release/*.wasm /tmp/.plotline/plugins
	@tar -czf plugins.tar.gz -C /tmp/.plotline/plugins .
	
	@rm -rf /tmp/.plotline/plugins

install:
	@mkdir -p ${PLUGINS_DIR}
	@tar -xzf plugins.tar.gz -C ${PLUGINS_DIR}

uninstall:
	@rm -rf ${PLUGINS_DIR}/*

clean:
	@rm -rf /tmp/.plotline/plugins
	@rm -rf plugins.tar.gz