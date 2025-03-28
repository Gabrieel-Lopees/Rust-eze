#!/bin/bash

echo "Oi, aqui está como está meu projeto agora:"

# Salvar saída em um arquivo temporário
tmp_file=$(mktemp)

for file in src/lib.rs src/main.rs src/movement.rs src/player.rs src/ui.rs src/walls.rs; do
    echo -e "\n### $file" | tee -a "$tmp_file"
    cat "$file" | tee -a "$tmp_file"
    echo -e "\n" | tee -a "$tmp_file"
done

# Copiar para a área de transferência usando xclip
cat "$tmp_file" | xclip -selection clipboard
echo "Conteúdo copiado para a área de transferência (X11)."

# Remover o arquivo temporário
rm "$tmp_file"

