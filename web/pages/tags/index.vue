<template>
  <div class="flex flex-col items-center justify-center gap-2 m-4 min-h-full">
    <span v-if="error" class="text-red-500">
      {{ error.message }}
    </span>
    <div class="flex flex-col gap-4">
      <NuxtLink v-for="tag in data?.tag?.nodes || []" :key="tag.id" class="flex gap-4 items-center" :to="`/tags/${tag.id}`">
        <img
          :src="'/assets/' + tag.icon"
          v-if="tag.icon"
          alt="Tag Icon"
          class="h-16 w-16 rounded"
        />
        <div v-else>
          <img
            src="~/assets/img/question.webp"
            alt="Placeholder Icon"
            class="h-16 w-16 rounded"
          />
        </div>
        <div>
          <h3 class="text-2xl">{{ tag.name }}</h3>
          <span v-if="tag.platformText">{{ tag.platformText }}</span>
          <p class="text-sm text-gray-500">
            Imported at: {{ new Date(tag.importedAt).toLocaleString() }}
          </p>
        </div>
      </NuxtLink>
    </div>
    <div class="mt-auto">
      <button
        @click="changePage(-1)"
        class="p-1 text-xl mr-2"
        :class="{
          invisible: page == 0,
        }"
      >
        <
      </button>
      <span
        >Page {{ page + 1 }} of
        {{ data?.tag?.paginationInfo?.pages }} ({{
          data?.tag?.paginationInfo?.total
        }}
        items)</span
      >
      <button
        v-if="page + 1 < data?.tag?.paginationInfo?.pages"
        @click="changePage(1)"
        class="p-1 text-xl ml-2"
      >
        >
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { graphql } from "~/utils/gql/gql";

const route = useRoute();
const page = computed(() => parseInt(route.query.page?.toString() || "0"));

const dataQuery = graphql(`
  query getTagData($page: Int!) {
    tag(
      pagination: { page: { limit: 8, page: $page } }
      orderBy: { name: ASC }
    ) {
      nodes {
        id
        name
        icon
        platformText
        importedAt
      }
      paginationInfo {
        pages
        total
      }
    }
  }
`);

const { data, error, refresh } = await useAsyncQuery(dataQuery, {
  page: page,
});

async function changePage(increment: number) {
  await navigateTo("/tags?page=" + (page.value + increment));
  await refresh();
}

definePageMeta({
  layout: "wrapper",
});
</script>
