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
  <v-card flat>
    <v-card-text>
      <v-table>
        <thead>
        <tr>
          <th class="text-left d-flex align-center">
            <v-checkbox-btn v-model="selectAll" class="pe-2"/>
            {{ $t("dashboard.schematic.name") }}
          </th>

          <th class="text-left">
            {{ $t("dashboard.schematic.id") }}
          </th>

          <th class="text-left">
            {{ $t("dashboard.schematic.actions") }}
          </th>
        </tr>
        </thead>

        <tbody>
        <tr v-for="entry in schematics" :key="entry.id">
          <td class="d-flex align-center">
            <v-checkbox-btn v-model="selected" :value="entry.id" class="pe-2"/>
            {{ entry.name }}
          </td>

          <td>
            {{ entry.id }}
          </td>

          <td>
            <v-icon icon="mdi-trash-can-outline" @click="deleteSchematic(entry.id)"/>
            <v-icon icon="mdi-download-outline" @click="downloadSchematic(entry.id)"/>
          </td>
        </tr>
        </tbody>
      </v-table>

      <v-expand-transition>
        <div v-if="selected.length > 0" class="mt-5 pb-2">
          <v-btn class="float-left" variant="outlined" color="error" @click="deleteSelectedSchematics">
            {{ $t("form.deleteSelected") }}
          </v-btn>

          <v-btn class="float-right" variant="outlined" color="green" @click="downloadSelectedSchematics">
            {{ $t("form.downloadSelected") }}
          </v-btn>
        </div>
      </v-expand-transition>

      <v-pagination v-if="pages > 1" v-model="page" :length="pages" class="mt-5"/>

      <DashboardAccountSchematicUpload @upload="fetchSchematics(page)"/>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import {ref, watch} from "#imports";
import {SchematicEntry} from "~/composables/types";
import Fetch from "~/composables/fetch";
import {useAuthStore} from "~/stores/auth";

const account = useAuthStore().account!;

const schematics = ref([] as SchematicEntry[]);
const selected = ref([] as string[]);
const pages = ref(0);
const total = ref(0);
const page = ref(1);
const selectAll = ref(false);

watch(selectAll, (newValue: boolean) => {
  if (newValue)
    selected.value = schematics.value.map((schematic) => schematic.id)
  else
    selected.value = []
})

/**
 * fetch the schematic entries
 */
async function fetchSchematics(page: number) {
  await Fetch.getPage<SchematicEntry>(`/account/${account.id}/schematic`, page)
      .then((response) => {
        schematics.value = response._data!.data;
        pages.value = response._data!.pages;
        total.value = response._data!.total;
      })
}

watch(page, async (page: number) => {
  await fetchSchematics(page);
  selected.value = [];
  selectAll.value = false;
}, {immediate: true})

/**
 * delete a schematic
 * @param schematicId {string} the schematic to delete
 */
async function deleteSchematic(schematicId: string) {
  await Fetch.delete(`/account/${account.id}/schematic/${schematicId}`, {})
      .then(async () => {
        await fetchSchematics(page.value)
        selected.value = selected.value.filter((id) => schematics.value.some((schem) => schem.id === id))
      })
}

/**
 * delete all selected schematics
 */
async function deleteSelectedSchematics() {
  for await(const schematicId of selected.value) {
    await deleteSchematic(schematicId)
  }
}

/**
 * download a schematic
 * @param schematicId {string} the schematic to download
 */
async function downloadSchematic(schematicId: string) {
  await Fetch.get(`/account/${account.id}/schematic/${schematicId}`, {})
      .then(async (response) => {
        // @ts-ignore
        const disposition = response.headers.get('content-disposition');
        const fileName = disposition.split("filename=")[1].replace("\\", "")

        // @ts-ignore
        const url = window.URL
            .createObjectURL(new Blob([response._data]));
        const link = document.createElement('a');
        link.href = url;
        // @ts-ignore
        link.setAttribute('download', fileName);
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
      })
}

/**
 * download all selected schematics
 */
async function downloadSelectedSchematics() {
  for await(const schematicId of selected.value) {
    await downloadSchematic(schematicId)
  }
}

</script>
