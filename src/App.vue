<script lang="tsx" setup>
import { symlinkStore } from './store/symlink_table.store';
import FileSelect from './parts/file-select.vue'
import FileTable from './parts/file-table.vue'

const store = symlinkStore()

</script>

<template>
  <div class="h-screen flex flex-col">
    <main class="bg-slate-200 flex-1 overflow-y-auto p-4 flex flex-col gap-1">
      <FileSelect v-model="store.link.source" name="原始路径" :btnclick="store.open_source"/>
      <FileSelect v-model="store.link.target" name="映射路径" :btnclick="store.open_target"/>
      <el-input  v-model="store.link.name" :disabled="store.link.diable_name">
        <template #prepend>
            文件名称
        </template>
      </el-input>
      <el-input  v-model="store.link.lname">
        <template #prepend>
            链接名称
        </template>
      </el-input>
      <div class="flex flex-row gap-1">
        <el-select
          v-model="store.link.type"
          @change="(v: number) => store.linkTypeSel[v].click()"
          style="width: 240px">
          <el-option
            v-for="item in store.linkTypeSel"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
        <el-button @click="store.symlink" type="primary">映射</el-button>
      </div>
      <div>
        <FileTable />
      </div>
    </main>
  </div>
</template>