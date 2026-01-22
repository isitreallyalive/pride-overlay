WEBP_COMMIT := 95ea5226c870449522240ccff26f0b006037c520

.PHONY: libwebp build
default: build

# build libwebp
libwebp:
	@if [ ! -d libwebp ]; then \
		mkdir -p libwebp; \
		curl -sL https://github.com/webmproject/libwebp/archive/$(WEBP_COMMIT).tar.gz | tar xz --strip 1 -C libwebp; \
		cd libwebp && ./autogen.sh && \
		emconfigure ./configure \
			--disable-shared \
			--enable-static \
			--disable-examples \
			--disable-tools \
			--disable-docs \
			--disable-manpages && \
		emmake make; \
	fi

build: libwebp
	@mkdir -p dist
	emcc -Ilibwebp/src --bind -O2 -sENVIRONMENT=worker -sEXPORT_ES6=1 -o dist/webp.js src/webp.cpp libwebp/src/.libs/libwebp.a libwebp/sharpyuv/.libs/libsharpyuv.a
	bun run esbuild.ts