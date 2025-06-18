FROM espressif/idf-rust:esp32_1.86.0.0

# root for apt
USER root

# QEMU dependencies
RUN apt-get update && apt install -y -q \
wget \
git \
libglib2.0-dev \
libfdt-dev \
libpixman-1-dev \
zlib1g-dev \
ninja-build \
libsdl2-dev \
libslirp-dev \
x11vnc \
xvfb

# QEMU
# https://github.com/espressif/qemu/releases/download/esp-develop-8.1.3-20231206/qemu-xtensa-softmmu-esp_develop_8.1.3_20231206-x86_64-linux-gnu.tar.xz
ENV QEMU_REL=esp-develop-9.2.2-20250228
ENV QEMU_SHA256=0ad2f51961ce8f0890c55474c70101201afc59c6464fe8502c39af523a641244
ENV QEMU_DIST=qemu-xtensa-softmmu-esp_develop_9.2.2_20250228-x86_64-linux-gnu.tar.xz
ENV QEMU_URL=https://github.com/espressif/qemu/releases/download/${QEMU_REL}/${QEMU_DIST}

RUN wget --no-verbose ${QEMU_URL} \
  && echo "${QEMU_SHA256} *${QEMU_DIST}" | sha256sum --check --strict - \
  && tar -xf $QEMU_DIST -C /opt \
  && rm ${QEMU_DIST}

ENV PATH=/opt/qemu/bin:${PATH}

# Back to esp, default user in idf-rust
USER esp

RUN mkdir ~/.vnc
RUN x11vnc -storepasswd esp ~/.vnc/passwd

CMD ["/bin/bash", "-c"]