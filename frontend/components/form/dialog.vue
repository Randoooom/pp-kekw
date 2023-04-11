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
  <v-dialog v-model="dialog" persistent>
    <v-card :max-width="props.maxWidth" class="ma-auto" rounded="lg" width="100vw">
      <v-card-title class="dialog-title-bg pt-3">
        <slot name="title">
          <span class="text-h3 font-intro-inline">
            {{ props.title }}
          </span>
        </slot>
      </v-card-title>

      <v-card-subtitle>
        <slot name="subtitle" />
      </v-card-subtitle>

      <v-card-text>
        <v-row>
          <slot />
        </v-row>
      </v-card-text>

      <v-card-actions>
        <slot name="actions" />
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts" setup>
import { computed } from "vue";


const props = defineProps({
  title: {
    type: String, required: false, default: ""
  },
  maxWidth: {
    type: String, required: false, default: "700px"
  }
});

const dialog = computed<boolean>(() => true);
</script>

<style lang="sass" scoped>
.dialog-title-bg
  background: linear-gradient(180deg, rgba(73, 0, 30, 1) 0%, rgba(140, 0, 0, 1) 100%)
</style>