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
  <v-app-bar>
    <v-container>
      <v-row class="align-center">
        <client-only>
          <v-app-bar-nav-icon v-if="isXs"/>
        </client-only>

        <v-app-bar-title class="d-flex align-center justify-start">
          <NuxtLink :to="localePath('/')" class="font-intro-inline text-white" id="home">
            <span :class="myClass">MY</span>PLAYPLANET.NET
          </NuxtLink>
        </v-app-bar-title>

        <v-spacer v-if="!isXs"/>

        <v-btn-toggle v-if="!isXs" v-model="route" selected-class="text-accent">
          <v-btn>
            <NuxtLink :to="localePath('/') + '#about'">
              {{ $t("nav.about") }}
            </NuxtLink>
          </v-btn>

          <v-btn>
            <NuxtLink :to="localePath('/events')">
              {{ $t("nav.events") }}
            </NuxtLink>
          </v-btn>
        </v-btn-toggle>

        <LayoutAccountAvatar class="ml-5" />
      </v-row>
    </v-container>
  </v-app-bar>
</template>

<script lang="ts" setup>
import {computed, useRoute} from "#imports";

const route = computed({
  get: () => {
    const route = useRoute().fullPath.toLowerCase();

    if (route.includes("#about")) return 0;
    if (route.includes("/events")) return 1;
    return -1;
  },
  set: (_value: number) => {
  }
});

const myClass = computed(() => {
  let accent = false;
  if (route.value === -1) accent = true;

  return {
    "text-accent": accent,
    "text-white": !accent,
  }
})
</script>

<style lang="sass" scoped>
#home
  font-size: 27px
</style>