# Sticker

App de desktop simples em Rust para guardar anotações ("stickers") em **JSON**, com edição e visualização em **Markdown**. GUI cross-platform usando `eframe`/`egui`.

- Lista, busca, cria, edita e remove stickers
- Editor com **preview Markdown ao vivo** lado a lado
- Cada sticker tem título, conteúdo (Markdown), tags e datas de criação/edição
- Persistência local em um único arquivo JSON
- Roda em **Linux** e **Windows**, binário único, sem runtime adicional

---

## Sumário

- [Pré-requisitos](#pré-requisitos)
- [Como rodar (Linux)](#como-rodar-linux)
- [Como rodar (Windows)](#como-rodar-windows)
- [Cross-compile do Linux para Windows](#cross-compile-do-linux-para-windows)
- [Tutorial de uso](#tutorial-de-uso)
- [Onde os dados ficam salvos](#onde-os-dados-ficam-salvos)
- [Estrutura do projeto](#estrutura-do-projeto)
- [Dicas de desenvolvimento](#dicas-de-desenvolvimento)
- [Solução de problemas](#solução-de-problemas)

---

## Pré-requisitos

Você precisa apenas do **Rust toolchain** (cargo + rustc 1.92+).

### Instalar Rust

**Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

**Windows:**
1. Baixe e execute `rustup-init.exe` em <https://rustup.rs>
2. Aceite os defaults (toolchain `stable-x86_64-pc-windows-msvc`)
3. Reabra o terminal (PowerShell ou Prompt de Comando) para o `cargo` ficar no PATH

Verifique:
```
cargo --version
rustc --version
```

---

## Como rodar (Linux)

```bash
git clone <este-repo> sticker-app
cd sticker-app

# Modo desenvolvimento (compilação rápida, mais lento em runtime)
cargo run

# Modo release (compila uma vez, executa fluido)
cargo run --release
```

O binário compilado fica em:
- Debug: `target/debug/sticker`
- Release: `target/release/sticker`

Você pode copiá-lo para qualquer lugar e executar:
```bash
cp target/release/sticker ~/.local/bin/
sticker
```

### Bibliotecas de sistema (caso o build reclame)

A maioria das distros já tem o necessário, mas se faltar algo:

**Arch / Manjaro:**
```bash
sudo pacman -S libxkbcommon wayland libxcb
```

**Debian / Ubuntu:**
```bash
sudo apt install libxkbcommon-dev libwayland-dev libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

**Fedora:**
```bash
sudo dnf install libxkbcommon-devel wayland-devel libxcb-devel
```

---

## Como rodar (Windows)

Em um **PowerShell** ou **Prompt de Comando**, dentro da pasta do projeto:

```powershell
cargo run --release
```

Na primeira execução o cargo baixa as dependências e compila — leva alguns minutos. Nas próximas, é instantâneo.

O executável fica em:
```
target\release\sticker.exe
```

Você pode dar duplo clique nesse `.exe` ou criar um atalho na área de trabalho. Não precisa do terminal nem do Rust instalado para rodar o `.exe` — ele é self-contained.

### Criar um atalho na área de trabalho (Windows)

1. Abra `target\release\` no Explorador de Arquivos
2. Clique com o botão direito em `sticker.exe` → **Enviar para → Área de trabalho (criar atalho)**
3. (Opcional) Renomeie o atalho para "Sticker"

### Toolchain MSVC vs GNU

Os defaults do `rustup` no Windows usam **MSVC**, que precisa do **Visual Studio Build Tools** (instalado junto pelo `rustup-init` se você aceitar o prompt). Se preferir não instalar o Visual Studio:

```powershell
rustup default stable-x86_64-pc-windows-gnu
```

Isso usa o backend GNU (MinGW). Não precisa do Visual Studio, mas o MSVC produz binários ligeiramente menores.

---

## Cross-compile do Linux para Windows

Útil se você desenvolve no Linux mas quer entregar o `.exe` para outra pessoa.

```bash
# Uma vez só: instala o target e o linker MinGW
rustup target add x86_64-pc-windows-gnu
sudo pacman -S mingw-w64-gcc          # Arch
# ou: sudo apt install mingw-w64       # Debian/Ubuntu

# Compila
cargo build --release --target x86_64-pc-windows-gnu
```

O `.exe` final fica em:
```
target/x86_64-pc-windows-gnu/release/sticker.exe
```

É um binário único. Copie para qualquer máquina Windows e execute — não precisa de instalador nem de runtime.

---

## Tutorial de uso

Quando você abre o app, vê três áreas:

```
┌──────────────────────────────────────────────────────────┐
│  ➕ Novo  │  📤 Exportar .md  │  filtro: [          ]   │  ← topo
├──────────┬───────────────────────────────────────────────┤
│ Stickers │                                               │
│          │                                               │
│ #1 nota  │              área principal                   │
│ #2 ideia │           (visualização ou edição)            │
│ #3 todo  │                                               │
│          │                                               │
├──────────┴───────────────────────────────────────────────┤
│  status: 3 stickers                                      │  ← rodapé
└──────────────────────────────────────────────────────────┘
```

### 1. Criar seu primeiro sticker

1. Clique em **➕ Novo** no topo
2. Preencha:
   - **Título** — ex.: `Receita de bolo`
   - **Tags** — separadas por vírgula, ex.: `comida, receita, doce`
   - **Conteúdo** — escreva em Markdown no painel da esquerda
3. Veja o **preview** atualizando ao vivo no painel da direita
4. Clique em **💾 Salvar**

Exemplo de conteúdo Markdown:
```markdown
## Bolo de chocolate

**Tempo:** 40 minutos

### Ingredientes
- 2 xícaras de farinha
- 1 xícara de açúcar
- 3 ovos

### Modo de preparo
1. Misture os secos
2. Adicione os ovos
3. Asse a 180°C por 30 minutos

> Dica: peneire a farinha antes
```

### 2. Editar um sticker existente

1. Clique no sticker desejado na lista à esquerda
2. Veja o conteúdo renderizado em Markdown na área principal
3. Clique em **✏ Editar** para entrar no modo de edição
4. Faça as alterações e clique em **💾 Salvar** (ou **Cancelar** para descartar)

A data de "editado em" é registrada automaticamente.

### 3. Buscar stickers

Digite no campo **filtro** no topo. A busca procura em **título**, **conteúdo** e **tags** (case-insensitive). A lista lateral filtra em tempo real.

### 4. Remover um sticker

1. Selecione o sticker
2. Clique em **🗑 Remover**
3. Confirme clicando em **sim** (ou **nao** para cancelar)

### 5. Exportar tudo para um arquivo Markdown

Clique em **📤 Exportar .md** no topo. O app gera um arquivo `stickers.md` ao lado do JSON com **todos os stickers concatenados** em formato Markdown — pronto para colar em um README, wiki, ou abrir no VS Code/Obsidian.

A barra de status mostra o caminho exato onde o arquivo foi salvo.

### 6. Sintaxe Markdown suportada

O preview renderiza CommonMark padrão:

| Sintaxe | Resultado |
|---|---|
| `# Título` | Cabeçalhos `#`, `##`, `###` |
| `**negrito**` | **negrito** |
| `*itálico*` | *itálico* |
| `` `código` `` | `código inline` |
| ` ```rust ... ``` ` | bloco de código com syntax highlight |
| `- item` | listas |
| `1. item` | listas numeradas |
| `> citação` | bloco de citação |
| `[link](url)` | links |
| `---` | linha horizontal |
| `\| col1 \| col2 \|` | tabelas |

---

## Onde os dados ficam salvos

O app guarda tudo em **um único arquivo JSON**:

| Sistema | Caminho |
|---|---|
| Linux | `~/.local/share/sticker/stickers.json` |
| Windows | `%APPDATA%\sticker\stickers.json` |
| macOS | `~/Library/Application Support/sticker/stickers.json` |

Você pode fazer **backup** simplesmente copiando esse arquivo, ou versioná-lo no Git/Dropbox para sincronizar entre máquinas.

### Usar um arquivo customizado

Defina a variável de ambiente `STICKER_FILE`:

**Linux/macOS:**
```bash
export STICKER_FILE=~/Documentos/minhas-notas.json
cargo run --release
```

**Windows (PowerShell):**
```powershell
$env:STICKER_FILE = "C:\Users\seu-usuario\Documents\minhas-notas.json"
cargo run --release
```

**Windows (Prompt de Comando):**
```cmd
set STICKER_FILE=C:\Users\seu-usuario\Documents\minhas-notas.json
cargo run --release
```

Útil para ter perfis separados (trabalho, pessoal) ou para testes em desenvolvimento.

### Formato do JSON

```json
{
  "next_id": 4,
  "stickers": [
    {
      "id": 1,
      "title": "comprar pao",
      "content": "padaria da esquina, integral",
      "tags": ["compras", "comida"],
      "created_at": "2026-04-29T22:58:43Z",
      "updated_at": null
    }
  ]
}
```

Como é texto puro, dá para editar manualmente em qualquer editor se precisar.

---

## Estrutura do projeto

```
sticker-app/
├── Cargo.toml          # dependências e metadados do crate
├── README.md           # este arquivo
└── src/
    ├── main.rs         # entry point + GUI (eframe/egui)
    └── storage.rs      # lógica de Sticker, Store, load/save, render Markdown
```

**Dependências:**
- `eframe` + `egui` — GUI cross-platform imediata
- `egui_commonmark` — renderização Markdown dentro do egui
- `serde` + `serde_json` — serialização JSON
- `chrono` — timestamps
- `dirs` — caminhos cross-platform de dados do usuário

---

## Dicas de desenvolvimento

```bash
cargo check              # valida tipos rapidamente, sem gerar binário
cargo run                # roda em modo debug
cargo run --release      # roda em modo release (mais fluido)
cargo build --release    # apenas compila o release
cargo clippy             # lints adicionais
cargo fmt                # formatação automática
```

**Iteração rápida na GUI:** use `cargo run` (debug) durante desenvolvimento — o tempo de compilação é menor. Use release apenas para entrega ou benchmark.

**Storage isolado para testes:**
```bash
STICKER_FILE=/tmp/sticker-dev.json cargo run
```
Reset:
```bash
rm -f /tmp/sticker-dev.json
```

---

## Solução de problemas

### "error: linker `cc` not found" no Linux
Falta o gcc/clang. Instale:
- Arch: `sudo pacman -S base-devel`
- Debian/Ubuntu: `sudo apt install build-essential`

### "error: Microsoft C++ Build Tools is required" no Windows
O toolchain MSVC precisa do Visual Studio Build Tools. Duas opções:
1. Instalar via <https://visualstudio.microsoft.com/visual-cpp-build-tools/> (selecione "Desktop development with C++")
2. Trocar para o toolchain GNU: `rustup default stable-x86_64-pc-windows-gnu`

### A janela não abre / fica preta no Linux
Provavelmente faltando bibliotecas Wayland/X11. Veja a seção [Bibliotecas de sistema](#bibliotecas-de-sistema-caso-o-build-reclame).

Como fallback, force X11:
```bash
WINIT_UNIX_BACKEND=x11 cargo run --release
```

### "permission denied" ao executar o binário no Linux
```bash
chmod +x target/release/sticker
```

### Quero apagar tudo e começar de novo
Apague o arquivo de dados:
- Linux: `rm ~/.local/share/sticker/stickers.json`
- Windows: `del %APPDATA%\sticker\stickers.json`

---

## Licença

Uso pessoal. Adapte conforme precisar.
