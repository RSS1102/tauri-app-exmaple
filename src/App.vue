<script setup lang="ts">
import { Menu, CheckMenuItem, IconMenuItem } from '@tauri-apps/api/menu';
import { Image } from '@tauri-apps/api/image';

//  todo: `icon`  `text`  `check` type change status

let currentLanguage = 'en';

const check_sub_item_en = await CheckMenuItem.new({
  id: 'en',
  text: 'English',
  checked: currentLanguage === 'en',
  action: () => {
    currentLanguage = 'en';
    check_sub_item_en.setChecked(currentLanguage === "en");
    check_sub_item_cn.setChecked(currentLanguage === "cn");

    console.log('English pressed');
  },
});

const check_sub_item_cn = await CheckMenuItem.new({
  id: 'cn',
  text: 'Chinese',
  checked: currentLanguage === 'cn',
  action: () => {
    currentLanguage = 'cn';
    check_sub_item_en.setChecked(currentLanguage === "en");
    check_sub_item_cn.setChecked(currentLanguage === "cn");

    console.log('Chinese pressed');
  },
});

// Load icon from path
const icon = await Image.fromPath('../src/icon.png')
console.log(icon);

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
        id: 'file',
        text: 'File',
        items: [
          {
            id: 'open',
            text: 'Open',
            action: () => {
              console.log('open pressed');
            },
          },
          {
            id: 'quit',
            text: 'Quit',
            action: () => {
              console.log('Quit pressed');
            },
          },
        ],
      },
      {
        id: 'language_items',
        text: 'language',
        items: [
          check_sub_item_en,
          check_sub_item_cn,
          icon_item,
        ],
      },
    ],
  });

  await menu.setAsAppMenu()

</script>

<template>
  <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>
</template>