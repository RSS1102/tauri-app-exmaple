<script setup lang="ts">
import { Menu, PredefinedMenuItem } from '@tauri-apps/api/menu';

const copy = await PredefinedMenuItem.new({
  text: 'copy-text',
  item: 'Copy',
});

const separator = await PredefinedMenuItem.new({
  text: 'separator-text',
  item: 'Separator',
});

const undo = await PredefinedMenuItem.new({
  text: 'undo-text',
  item: 'Undo',
});

const redo = await PredefinedMenuItem.new({
  text: 'redo-text',
  item: 'Redo',
});

const cut = await PredefinedMenuItem.new({
  text: 'cut-text',
  item: 'Cut',
});

const paste = await PredefinedMenuItem.new({
  text: 'paste-text',
  item: 'Paste',
});

const select_all = await PredefinedMenuItem.new({
  text: 'select_all-text',
  item: 'SelectAll',
});

const menu = await Menu.new({
  items: [
    copy,
    separator,
    undo,
    redo,
    cut,
    paste,
    select_all,
  ],
});

await menu.setAsAppMenu()

</script>

<template>
  <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>
</template>