# bevy-nodemoui
say no to demoui

## Development
To develop inside devcontainer you must:
- Have the newest drivers for your GPU
- There's a bunch of dependencies on the [Dockerfile](./.devcontainer/Dockerfile) and configurations on the [devcontainer.json](./.devcontainer/devcontainer.json) 
- Install NVIDIA Container Toolkit
```bash
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg \
&& curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
sudo apt-get install -y nvidia-container-toolkit
```
- Enable container runkit docker runtime `sudo nvidia-ctk runtime configure --runtime=docker`
- Enable X11 forwarding `xhost +`, remember to disable later `xhost -`