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
  <v-expand-x-transition>
    <v-navigation-drawer v-model="expanded" :mini-variant="!isXs"
                         :permanent="!isXs" :rail="!isXs" fixed>
      <v-list density="compact" nav>
        <template v-if="isXs">
          <v-list-item class="d-flex justify-center"
                       nuxt
                       prepend-icon="mdi-information-outline" :to="localePath('/') + '#about'">
            <v-list-item-title>
              {{ $t("nav.about") }}
            </v-list-item-title>
          </v-list-item>

          <v-list-item class="d-flex justify-center"
                       nuxt
                       prepend-icon="mdi-calendar" :to="localePath('/events')">
            <v-list-item-title>
              {{ $t("nav.events") }}
            </v-list-item-title>
          </v-list-item>

          <v-divider/>
        </template>

        <v-divider class="mt-2 mb-2"/>

        <v-list-item v-if="auth.hasPermission('news')" class="d-flex justify-center"
                     nuxt
                     prepend-icon="mdi-newspaper-variant-outline" :to="localePath('/news')">
          <v-list-item-title>
            {{ $t("nav.news") }}
          </v-list-item-title>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
  </v-expand-x-transition>
</template>

<script lang="ts" setup>
import {computed, isXs} from "#imports"
import {useAuthStore} from "~/stores/auth";

const auth = useAuthStore();

const expanded = computed(() => {
  return isXs.value || auth.permissions.length > 0
});
</script>

<style lang="sass">
.v-navigation-drawer__content
  display: flex
  justify-content: center
  flex-grow: 0
  flex-direction: column

  .v-list
    .v-list-item
      .v-list-item-title
        width: 175px

      .v-list-item__prepend
        .v-icon
          @media screen and (min-width: 600px)
            margin: 0 20px !important

        .v-avatar
          @media screen and (max-width: 600px)
            margin-right: 24px !important
</style>