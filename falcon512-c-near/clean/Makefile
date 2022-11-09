# Build script for the Falcon implementation.
#
# ==========================(LICENSE BEGIN)============================
#
# Copyright (c) 2017-2019  Falcon Project
#
# Permission is hereby granted, free of charge, to any person obtaining
# a copy of this software and associated documentation files (the
# "Software"), to deal in the Software without restriction, including
# without limitation the rights to use, copy, modify, merge, publish,
# distribute, sublicense, and/or sell copies of the Software, and to
# permit persons to whom the Software is furnished to do so, subject to
# the following conditions:
#
# The above copyright notice and this permission notice shall be
# included in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
# IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
# CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
# TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
# SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
#
# ===========================(LICENSE END)=============================
#
# @author   Thomas Pornin <thomas.pornin@nccgroup.com>

.POSIX:

# =====================================================================
#
# Configurable options:
#   CC       C compiler; GCC or Clang are fine; MSVC (2015+) works too.
#   CFLAGS   Compilation flags:
#             * Optimization level -O2 or higher is recommended
#            See config.h for some possible configuration macros.
#   LD       Linker; normally the same command as the compiler.
#   LDFLAGS  Linker options, not counting the extra libs.
#   LIBS     Extra libraries for linking:
#             * If using the native FPU, test_falcon and application
#               code that calls this library may need: -lm
#               (normally not needed on x86, both 32-bit and 64-bit)

CC = clang
CFLAGS = -Wall -Wextra -Wshadow -Wundef -O3 -fPIC#-pg -fno-pie
LD = clang
LDFLAGS = -shared #-pg -no-pie
LIBS = #-lm

CFLAGS1 = -Wall -Wextra -Wshadow -Wundef -O3#-pg -fno-pie
# =====================================================================

OBJ = build/codec.o build/common.o build/falcon.o build/fft.o build/fpr.o build/keygen.o build/rng.o build/shake.o build/sign.o build/vrfy.o

all: build libfalcon512clean test_falcon

build:
	-mkdir build

clean:
	-rm -f $(OBJ) build/libfalcon512clean build/test_falcon.o build/test_falcon

clean_test:
	-rm -f build/test_falcon.o build/test_falcon

libfalcon512clean: $(OBJ)
	$(LD) $(LDFLAGS) -o build/libfalcon512clean $(OBJ) $(LIBS)

test_falcon: $(OBJ) build/test_falcon.o
	$(LD) -o build/test_falcon $(OBJ) build/test_falcon.o

build/test_falcon.o: test/test_falcon.c falcon.h config.h inner.h fpr.h
	$(CC) $(CFLAGS1) -c -o build/test_falcon.o test/test_falcon.c

build/codec.o: codec.c config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/codec.o codec.c

build/common.o: common.c config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/common.o common.c

build/falcon.o: falcon.c falcon.h config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/falcon.o falcon.c

build/fft.o: fft.c config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/fft.o fft.c

build/fpr.o: fpr.c config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/fpr.o fpr.c

build/keygen.o: keygen.c config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/keygen.o keygen.c

build/rng.o: rng.c config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/rng.o rng.c

build/shake.o: shake.c config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/shake.o shake.c

build/sign.o: sign.c config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/sign.o sign.c

build/vrfy.o: vrfy.c config.h inner.h fpr.h
	$(CC) $(CFLAGS) -c -o build/vrfy.o vrfy.c
