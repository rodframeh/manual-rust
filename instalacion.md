# Instalación
## Instalación
- Archivos de comandos para descargar e instalar el lenguaje Rust
``` $ curl https://sh.rustup.rs -sSf | sh ```
- Si aún no va a cerrar la sesión, agregue los ejecutables del lenguaje Rust a la ruta del sistema manualmente
``` $ source $HOME/.cargo/env ```
- Alternativamente agregamos los ejecutables del lenguaje Rust a la ruta del perfil (es similar al anterior paso).
``` $ export PATH="$HOME/.cargo/bin:$PATH" ```
## Verficando la versión del compilador
``` $ rustc --version ```
## Verficando la versión de cargo
``` $ cargo --version ```
## Actualización
``` $ rustup update ```
## Desinstalar
``` $ rustup self uninstall ```