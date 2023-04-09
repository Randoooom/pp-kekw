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
  <div>
    <v-parallax id="about" src="images/about.png" height="950">
      <v-row class="mt-12 pt-12">
        <v-col cols="12"
               class="cover d-flex justify-center align-center text-h1 text-center font-intro-inline text-white">
          {{ $t("nav.about") }}
        </v-col>
      </v-row>

      <v-container id="about-timeline">
        <v-timeline align="start">
          <v-timeline-item icon="mdi-account-multiple" fill-dot dot-color="accent">
            <v-card>
              <v-card-title class="text-h6 bg-accent text-grey-lighten-3">
                {{ $t("about.community.title") }}
              </v-card-title>

              <v-card-text class="text-grey-lighten-2 pt-5">
                {{ $t("about.community.content") }}
              </v-card-text>
            </v-card>
          </v-timeline-item>

          <v-timeline-item icon="mdi-server-network" fill-dot dot-color="grey-lighten-2">
            <v-card>
              <v-card-title class="text-h6 bg-grey-lighten-2 text-grey-darken-3">
                {{ $t("about.server.title") }}
              </v-card-title>

              <v-card-text class="text-grey-lighten-2 pt-5">
                {{ $t("about.server.content") }}
              </v-card-text>

              <v-card-actions>
                <v-chip color="green-darken-2">
                  <v-icon icon="mdi-tag" class="mr-2"/>
                  1.19.2
                </v-chip>

                <v-chip color="grey" class="ml-2">
                  <v-icon icon="mdi-ip" class="mr-2"/>
                  myplayplanet.net
                </v-chip>
              </v-card-actions>
            </v-card>
          </v-timeline-item>

          <v-timeline-item icon="mdi-message-text-outline" fill-dot dot-color="accent">
            <v-card>
              <v-card-title class="text-h6 bg-accent text-grey-lighten-3">
                {{ $t("about.forum.title") }}
              </v-card-title>

              <v-card-text class="text-grey-lighten-2 pt-5">
                {{ $t("about.forum.content") }}
              </v-card-text>

              <v-card-actions>
                <v-chip color="grey" class="ml-2">
                  <v-icon icon="mdi-domain" class="mr-2"/>
                  forum.myplayplanet.net
                </v-chip>
              </v-card-actions>
            </v-card>
          </v-timeline-item>
        </v-timeline>
      </v-container>
    </v-parallax>
  </div>
</template>

<script lang="ts" setup>

</script>

<style lang="sass">
#about
  img
    filter: opacity(0.4)

#about-timeline
  max-width: 1000px
</style>
