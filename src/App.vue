<script setup lang="ts">
import { Menu, CheckMenuItem, IconMenuItem } from '@tauri-apps/api/menu';
import { Image } from '@tauri-apps/api/image';

let currentLanguage = 'en';

const check_sub_item_1 = await CheckMenuItem.new({
  id: 'check_sub_item_1',
  text: 'Check Sub Item 2',
  checked: true,
  action: () => {
    console.log('chinese pressed');
    currentLanguage = 'en';
  },
});

const check_sub_item_2 = await CheckMenuItem.new({
  id: 'check_sub_item_2',
  text: 'Check Sub Item 2',
  checked: false,
  action: () => {
    console.log('chinese pressed');
    currentLanguage = 'cn';
  },
});

const icon = await Image.fromPath('../src/icon.png')
console.log(icon);
// this will not work.
const icon_item = await IconMenuItem.new({
  id: 'icon_item',
  text: 'Icon Item',
  icon: icon,
  action: () => {
    console.log('icon pressed');
  },
});

const menu = await Menu.new({
  items: [
    {
      id: 'quit',
      text: 'Quit',
      action: () => {
        console.log('quit pressed');
      },
    },
    {
      id: 'close',
      text: 'Close',
      action: () => {
        console.log('close pressed');
      },
    },
    {
      id: 'language_items',
      text: 'language',
      items: [
        check_sub_item_1,
        check_sub_item_2,
        icon_item,
      ],
    },
  ],
});

menu.setAsAppMenu().then(res => {
  console.log("menu set success", res);
});

</script>

<template>
  <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>
</template>