<template>
  <div class="flex flex-col items-center justify-center gap-2 m-4">
    <span v-if="error" class="text-red-500">
      {{ error.message }}
    </span>
    <div class="flex flex-col gap-4">
      <NuxtLink
        v-for="item in data?.contentItem.nodes || []"
        :key="item.id"
        class="flex gap-4 items-center"
        :to="'/content/' + item.id"
      >
        <img
          :src="'/assets/' + item.thumbnail"
          v-if="item.thumbnail"
          alt="Tag Icon"
          class="max-h-32 max-w-32 rounded"
        />
        <div v-else>
          <img
            src="~/assets/img/question.webp"
            alt="Placeholder Icon"
            class="h-32 w-32 rounded"
          />
        </div>
        <div>
          <h3 class="text-2xl">{{ item.title }}</h3>
          <p class="text-sm text-gray-500">
            Imported at: {{ new Date(item.importedAt).toLocaleString() }}
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
        {{ data?.contentItem.paginationInfo?.pages }} ({{
          data?.contentItem.paginationInfo?.total
        }}
        items)</span
      >
      <button
        v-if="
          page + 1 < data?.contentItem?.paginationInfo?.pages
        "
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
  query getAllContents($page: Int!) {
    contentItem(pagination: { page: { limit: 8, page: $page } }, orderBy: { importedAt: ASC }) {
      nodes {
        id
        thumbnail
        title
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
  await navigateTo("/content?page=" + (page.value + increment));
  await refresh();
}

definePageMeta({
  layout: "wrapper",
});
</script>
