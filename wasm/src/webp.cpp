#include "emscripten/bind.h"
#include "emscripten/val.h"
#include "webp/decode.h"
#include "webp/encode.h"

using namespace emscripten;

thread_local const val Uint8Array = val::global("Uint8Array");
thread_local const val Uint8ClampedArray = val::global("Uint8ClampedArray");
thread_local const val ImageData = val::global("ImageData");

val decode(std::string buffer)
{
  int width, height;
  std::unique_ptr<uint8_t[]> rgba(WebPDecodeRGBA((const uint8_t *)buffer.c_str(), buffer.size(), &width, &height));

  return rgba ? ImageData.new_(Uint8ClampedArray.new_(typed_memory_view(width * height * 4, rgba.get())), width, height) : val::null();
}

val encode(std::string img, int width, int height)
{
  auto img_in = (uint8_t *)img.c_str();

  WebPPicture pic;
  WebPMemoryWriter wrt;
  int ok;

  if (!WebPPictureInit(&pic))
  {
    // shouldn't happen, except if system installation is broken
    return val::null();
  }

  WebPConfig config;
  WebPConfigInit(&config);
  config.qmax = 100;

  // only use use_argb if we really need it, as it's slower
  pic.use_argb = config.lossless || config.use_sharp_yuv || config.preprocessing > 0;
  pic.width = width;
  pic.height = height;
  pic.writer = WebPMemoryWrite;
  pic.custom_ptr = &wrt;

  WebPMemoryWriterInit(&wrt);

  ok = WebPPictureImportRGBA(&pic, img_in, width * 4) && WebPEncode(&config, &pic);
  WebPPictureFree(&pic);
  val js_result = ok ? Uint8Array.new_(typed_memory_view(wrt.size, wrt.mem)) : val::null();
  WebPMemoryWriterClear(&wrt);
  return js_result;
}

EMSCRIPTEN_BINDINGS(webp)
{
  function("decode", &decode);
  function("encode", &encode);
}