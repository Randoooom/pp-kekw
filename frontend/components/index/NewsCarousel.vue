<!--
  -
  - The MIT License (MIT)
  -
  - Copyright (c) 2023 Fritz Ochsmann
  -
  - Permission is hereby granted, free of charge, to any person obtaining a copy
  - of this software and associated documentation files (the "Software"), to deal
  - in the Software without restriction, including without limitation the rights
  - to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  - copies of the Software, and to permit persons to whom the Software is
  - furnished to do so, subject to the following conditions:
  -
  - The above copyright notice and this permission notice shall be included in all
  - copies or substantial portions of the Software.
  -
  - THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  - IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  - FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  - AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  - LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  - OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  - SOFTWARE.
  -
  -->

<template>
  <v-carousel class="cover-screen" cycle interval="10000" progress="primary" show-arrows>
    <template v-slot:prev="{ props }">
      <v-btn variant="plain" rounded @click="props.onClick" :size="50">
        <v-icon icon="mdi-arrow-left" :size="40"/>
      </v-btn>
    </template>

    <template v-slot:next="{ props }">
      <v-btn variant="plain" rounded @click="props.onClick" :size="50">
        <v-icon icon="mdi-arrow-right" :size="40"/>
      </v-btn>
    </template>

    <v-carousel-item v-for="entry in news" :key="entry.id"
                     :src="`${runtimeConfig.public.apiBase}/cdn/news/${entry.id}.png`" cover>
      <div class="cover d-flex align-center justify-center flex-column font-intro-inline">
        <span class="text-h1 font-intro-inline">
          {{ entry.title }}
        </span>

        <div v-html="entry.content"/>
      </div>
    </v-carousel-item>
  </v-carousel>
</template>

<script lang="ts" setup>
import Fetch from "~/composables/fetch";
import {News} from "~/composables/types";
import {ref, useRuntimeConfig} from "#imports";

const news = ref([] as News[]);
await Fetch.get<News[]>("/news/shown")
    .then(response => (news.value = response._data!))

const runtimeConfig = useRuntimeConfig()
</script>