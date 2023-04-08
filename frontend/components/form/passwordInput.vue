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
  <v-text-field v-model="password" :append-inner-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
                :data-form-type="props.dataFormType"
                :hint="props.strength ? score : undefined"
                :label="props.label" :rules="[required()]"
                :type='show ? "text" : "password"' filled v-bind="$attrs" @click:append-inner="show = !show"/>
</template>

<script lang="ts" setup>
import {computed} from "vue";
import {ref, required, useI18n} from "#imports";
import zxcvbn from "zxcvbn";

const { t } = useI18n();
const password = ref("");
const show = ref(false);

const props = defineProps({
  strength: {type: Boolean, required: false, default: false},
  label: {type: String, required: false, default: "Passwort"},
  dataFormType: {type: String, required: false, default: "password"}
});

const score = computed<string>(() => {
  switch (zxcvbn(password.value).score) {
    case 0:
      return t("form.password.veryWeak");
    case 1:
      return t("form.password.weak");
    case 2:
      return t("form.password.medium");
    case 3:
      return t("form.password.strong");
    case 4:
      return t("form.password.veryStrong");
  }
});
</script>