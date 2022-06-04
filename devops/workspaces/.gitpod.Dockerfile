FROM gitpod/workspace-full

COPY scripts /tmp/library-scripts/
COPY rust-toolchain.toml .

# gitpod default user is gotpod, i don't want to use SUDO  in shell scripts cos it breaks the vscode
# think about this later
USER root

RUN bash /tmp/library-scripts/install-deps.sh \
	&& bash /tmp/library-scripts/setup-rust-related-pacakges.sh \
	&& bash /tmp/library-scripts/install-common-crates.sh 

RUN curl https://ipfs.anagolay.network/ipfs/bafybeifnjzcbu76ivm22w3x37pnqqnha753ity5divosgvdnfu5ybsquji > /usr/local/bin/ipfsCli \
&& chmod +x /usr/local/bin/ipfsCli \
&& ipfsCli -V

# swtich back to the gitpod user
USER gitpod 

RUN brew install fzf

# run the installation script  
RUN sh -c "$(wget https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh -O -)"

# install powerlevel10k
RUN git clone https://github.com/romkatv/powerlevel10k.git ~/.oh-my-zsh/custom/themes/powerlevel10k
# Syntax Plugin
RUN git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
# Autocomplete
RUN git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
