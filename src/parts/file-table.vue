<template>
  <div class="flex flex-col gap-1">
    <div class="flex flex-row">
      <el-button @click="store.read_symlink_data">读取</el-button>
      <el-button type="danger" @click="getSelectEvent">删除所选</el-button>
    </div>
    <vxe-table
      round
      align="left"
      ref="tableRef"
      :expand-config="tableExpand"
      :data="store.symlink_data.data">
      <vxe-column type="expand" title="" width="30">
        <template #content="{ row }">
          <div class="p-4 box-border bg-slate-100">
            <vxe-grid :columns="files_columns" :data="row.files"></vxe-grid>
          </div>
        </template>
      </vxe-column>
      <vxe-column type="checkbox" title="" width="50"></vxe-column>
      <vxe-column type="seq" :title="store.symlink_data.data.length.toString()" width="50"></vxe-column>
      <vxe-column 
        sortable
        sort-type="string"
        show-overflow 
        v-for="col in columns" 
        :field="col.key" 
        :title="col.title"
        :min-width="col.min_width" />
    </vxe-table>
  </div>
</template>

<script lang="ts" setup>
import { symlinkData, symlinkStore } from '../store/symlink_table.store';
import { reactive, ref } from 'vue'
import { VxeColumnProps, VxeTableInstance, VxeTablePropTypes } from 'vxe-table'

const store = symlinkStore()
store.read_symlink_data()

const tableRef = ref<VxeTableInstance<symlinkData>>()
const columns = [
  { key: "name", title: "链接名称", min_width: 80 },
  { key: "source", title: "原始路径", min_width: "25%" },
  { key: "target", title: "目标路径", min_width: "25%" },
  { key: "time", title: "建立时间", min_width: 100 },
  { key: "type_", title: "链接类型", min_width: 100 },
];

const files_columns: VxeColumnProps[] = reactive([
  { type: 'seq', title: '序号', width: 60 },
  { field: "type_", title: "链接类型", width: 80 },
  { field: "source", title: "原始路径" },
  { field: "target", title: "目标路径" },
])

const tableExpand = ref<VxeTablePropTypes.ExpandConfig<symlinkData>>({
  lazy: true,
  async loadMethod ({ row }) {
    row.files = await store.read_symlink_files_data(row.files_id, false) as symlinkData[]
    console.log(row.files)
    files_columns[0].title = row.files.length.toString()
  }
})

const getSelectEvent = () => {
  const $table = tableRef.value
  if ($table) {
    const selectRecords = $table.getCheckboxRecords()
    console.log(selectRecords)
    store.remove_symlink(selectRecords)
  }
}
</script>