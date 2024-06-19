<template>
    <el-input v-model="path">
        <template #prepend>
            {{ props.name }}
        </template>
        <template #append>
            <el-button :icon="Folder" @click="props.btnclick" />
        </template>
    </el-input>
</template>
<script setup lang="tsx">
import { Folder } from "@element-plus/icons-vue";

const path = defineModel<string[] | string>({ required: true })

const props = defineProps<{
    name: string,
    btnclick: () => void
}>()


</script>