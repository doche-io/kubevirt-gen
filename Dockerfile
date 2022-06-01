FROM archlinux:latest
RUN pacman -Syyu --noconfirm
RUN pacman -S wget --noconfirm
RUN wget https://github.com/doche-io/kubevirt-gen/releases/download/v1.0.0/kubevirt-gen
RUN chmod 755 ./kubevirt-gen
CMD ./kubevirt-gen --port 3000 --bind 0.0.0.0
EXPOSE 3000

