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
  <div class="mt-15">
    <v-card-subtitle>
      {{ $t("dashboard.schematicUpload") }}
    </v-card-subtitle>

    <v-divider/>

    <FormFileInput v-model="files" accept=".schem,.schematic" color="accent" class="ma-5" multiple
                   prepend-icon="mdi-cube-outline"
                   :label="$t('form.selectFiles')"/>
    <v-btn variant="tonal" block color="green" :disabled="files.length === 0" @click="upload" :loading="loading">
      {{ $t("form.upload") }}

      <template #loader>
        <v-progress-circular color="accent" v-model="progress"/>
      </template>
    </v-btn>
  </div>
</template>

<script lang="ts" setup>
import {ref, useI18n} from "#imports";
import Fetch from "~/composables/fetch";
import {useAuthStore} from "~/stores/auth";
import {useEmitter} from "~/stores/emitter";

const { t } = useI18n();
const emit = defineEmits(["upload"])
const account = useAuthStore().account!;

const files = ref([] as File[]);
const progress = ref(0);
const loading = ref(false);

/**
 * upload the selected files
 */
async function upload() {
  loading.value = true;
  const progressPerFile = 100 / files.value.length;

  for await (const file of files.value) {
    await Fetch.post(`/account/${account.id}/schematic/upload/${file.name}`, {
      body: await file.arrayBuffer()
    })

    progress.value += progressPerFile
  }
  progress.value = 100;

  useEmitter().emit({
    icon: "mdi-upload-outline",
    content: "dashboard.schematicUploadComplete",
    color: "success",
    callback: async () => {
      progress.value = 0;
      loading.value = false;
      files.value = [];

      useEmitter().clear();
    }
  });

  emit("upload")
}
</script>
