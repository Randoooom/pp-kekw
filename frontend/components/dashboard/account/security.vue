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
      <v-row>
        <v-card-subtitle>
          {{ $t("dashboard.changePassword") }}
        </v-card-subtitle>
        <v-divider/>

        <v-col cols="12">
          <FormPasswordInput v-model="password" :label="$t('auth.password')"/>
        </v-col>

        <v-col cols="12">
          <FormPasswordInput v-model="newPassword" :label="$t('auth.newPassword')" strength/>
        </v-col>

        <v-col cols="12">
          <FormPasswordInput v-model="confirmNewPassword" :label="$t('auth.confirmNewPassword')"
                             @keyup.enter="tryChangePassword"/>
        </v-col>

        <v-col cols="12" v-if="account.totp">
          <v-text-field v-model="token" :label="$t('auth.2fa')" @keyup.enter="tryChangePassword"/>
        </v-col>

        <v-col cols="12">
          <v-btn variant="tonal" color="green" @click="changePassword" :loading="loading" :disabled="!canChangePassword" block>
            {{ $t("form.save") }}
          </v-btn>
        </v-col>

        <v-card-subtitle class="mt-5">
          {{ $t("auth.2fa") }}
        </v-card-subtitle>
        <v-divider/>

        <template v-if="!account.totp">
          <v-col cols="12">
            <v-alert color="grey-darken-2" icon="mdi-alert" class="text-white">
              {{ $t("dashboard.2faDisabled") }}
            </v-alert>
          </v-col>

          <v-col cols="12">
            <v-btn v-if="!account.totp" @click="openTotpDialog(true)" variant="tonal" color="green" block>
              {{ $t("form.activate") }}
            </v-btn>
          </v-col>
        </template>

        <template v-else>
          <v-col cols="12">
            <v-alert color="green-darken-3" icon="mdi-security">
              {{ $t("dashboard.2faActive") }}
            </v-alert>
          </v-col>

          <v-col cols="12">
            <v-btn @click="openTotpDialog(false)" variant="tonal" color="error" block>
              {{ $t("form.deactivate") }}
            </v-btn>
          </v-col>
        </template>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import {useAuthStore} from "~/stores/auth";
import Fetch from "~/composables/fetch";
import {computed, openTotpDialog, useI18n} from "#imports";
import {useEmitter} from "~/stores/emitter";

const account = useAuthStore().account!;
const {t} = useI18n();
const emitter = useEmitter();

const password = ref("");
const newPassword = ref("");
const confirmNewPassword = ref("");
const token = ref("");
const loading = ref(false);

const canChangePassword = computed(() => (!!password.value && !!newPassword.value
    && newPassword.value === confirmNewPassword.value
    && (account.totp && !!token.value || !account.totp)))

/**
 * try to change the password based on the computed props
 */
async function tryChangePassword() {
  if (canChangePassword.value)
    await changePassword()
}

/**
 * change the password
 */
async function changePassword() {
  loading.value = true;
  await Fetch.put("/auth/password", {
    body: {
      newPassword: newPassword.value,
      oldPassword: password.value,
      token: token.value
    }
  }).then(() => {
    loading.value = false;
    emitter.emit({
      color: "success",
      buttonText: t("emit.button"),
      content: t("auth.passwordChange.success"),
      icon: "mdi-check",
      callback: async () => emitter.clear()
    })

    newPassword.value = "";
    password.value = "";
    confirmNewPassword.value = "";
    token.value = "";
  }).catch(() => {
    loading.value = false;
    emitter.emit({
      color: "error",
      buttonText: t("emit.button"),
      content: t("auth.passwordChange.error"),
      icon: "mdi-alert-circle-outline",
      callback: async () => emitter.clear()
    })
  })
}
</script>

<style lang="sass" scoped>

</style>