<template>
  <el-table
    ref="refFormTable"
    row-key="originField"
    class="form-table-config"
    :data="formTableData"
    border
    @selection-change="handleFormSelection"
  >
    <el-table-column prop="tableName" label="表名" align="center" width="120">
      <template #default="{ row }">
        <el-input v-model="row.tableName" placeholder="tableName" />
      </template>
    </el-table-column>
    <el-table-column prop="originField" label="字段名" align="center" width="120">
      <template #default="{ row }">
        <el-input v-model="row.originField" placeholder="字段名" />
      </template>
    </el-table-column>
    <el-table-column prop="desc" label="字段描述" width="120">
      <template #default="{ row }">
        <el-input v-model="row.desc" placeholder="字段描述" />
      </template>
    </el-table-column>
    <el-table-column prop="componentType" align="center" label="组件类型" width="400">
      <template #default="{ row }">
        <el-radio-group v-model="row.componentType">
          <el-radio
            v-for="(item, index) in componentTypeArr"
            :key="index"
            :label="item.label"
            @click="chooseRowHandle(row)"
          >
            {{ item.title }}
          </el-radio>
        </el-radio-group>
      </template>
    </el-table-column>
    <el-table-column prop="width" align="center" label="宽度" width="60">
      <template #default="{ row }">
        <el-input v-model="row.width" placeholder="控件的宽度" />
      </template>
    </el-table-column>
    <el-table-column prop="rule" align="center" label="校验规则" width="100">
      <template #default="{ row }">
        <el-radio-group v-model="row.rule">
          <el-radio v-for="(item, index) in ruleMapping" :key="index" :label="item.key">
            {{ item.label }}
          </el-radio>
        </el-radio-group>
      </template>
    </el-table-column>
    <el-table-column align="center" prop="api" label="额外配置(select,cascader)" width="180">
      <template #default="{ row }">
        <div class="text-left">
          <el-input
            v-if="['selectApi', 'cascaderApi'].includes(row.componentType)"
            v-model="row.api"
            type="textarea"
            rows="2"
            placeholder="请求地址"
          />
          <el-input
            v-if="['selectApi', 'cascaderApi'].includes(row.componentType)"
            v-model="row.method"
            class="mtPx-4 widthPx-100"
            placeholder="请求方法"
          />
          <el-input
            v-if="['selectApi', 'cascaderApi'].includes(row.componentType)"
            v-model="row.labelKey"
            class="mtPx-4 widthPx-100"
            placeholder="label-key"
          />
          <el-input
            v-if="['selectApi', 'cascaderApi'].includes(row.componentType)"
            v-model="row.valueKey"
            class="mtPx-4 widthPx-100"
            placeholder="value-key"
          />
          <el-input
            v-if="['select', 'radio', 'checkbox', 'switch'].includes(row.componentType)"
            v-model="row.optionData"
            class="mtPx-4"
            type="textarea"
            rows="2"
            placeholder="数据枚举"
          />
          <!--cascaderApi  -->
          <el-input
            v-if="['cascaderApi'].includes(row.componentType)"
            v-model="row.children"
            class="mtPx-4 widthPx-100"
            placeholder="childrenKey"
          />
        </div>
      </template>
    </el-table-column>

    <el-table-column prop="width" align="center" label="操作" width="50">
      <template #default="{ row, $index }">
        <el-button text @click="deleteFormItem(row, $index)">删除</el-button>
      </template>
    </el-table-column>
  </el-table>
</template>

<script setup>
import {
  changeDashToCase,
  changeTheFirstWordToCase,
  componentTypeArr,
  componentTypeMapping,
  tbTypeMapping,
  ruleMapping,
  splitDescReturnDesc,
  splitDescReturnOptionData,
  splitTheOptionArr,
  changeDashToCaseAndFirstWord
} from './generatorUtis'
import commonUtil from '@/utils/commonUtil'
const setFormTableData = (checkColumnArr) => {
  checkColumnArr.forEach((fItem) => {
    const hasKey = commonUtil.findArrObjByKey(formTableData, 'columnName', fItem.columnName)
    if (!hasKey) {
      fItem.field = changeDashToCase(fItem.columnName) //_转驼峰
      fItem.fieldCase = changeDashToCaseAndFirstWord(fItem.columnName) //_转驼峰
      fItem.originField = fItem.columnName
      fItem.tbName = fItem.columnName

      fItem.type = tbTypeMapping(fItem.dataType) //数据库和java中的类型做映射
      fItem.componentType = componentTypeMapping(fItem.dataType, fItem.columnComment, fItem.columnName) //数据库和前端控件中的类型做映射
      fItem.rule = 'isNotNull'
      fItem.value = 'value'
      fItem.label = 'label'
      fItem.children = 'children'
      fItem.width = 120

      fItem.desc = splitDescReturnDesc(fItem.columnComment)
      fItem.optionData = splitDescReturnOptionData(fItem.columnComment)
      //api
      fItem.api = ''
      fItem.method = 'get'
      fItem.labelKey = 'name'
      fItem.valueKey = 'code'
      formTableData.push(fItem)
    }
  })
}
/*查询配置*/
let currentChooseRow = $ref({})
const chooseRowHandle = (row) => {
  currentChooseRow = row
}
let formTableData = $ref([])
let formSelection = $ref([])
const handleFormSelection = (val) => {
  formSelection = val
}
//删除和新增
const deleteFormItem = (row, index) => {
  formTableData.splice(index, 1)
}
//实现表格拖拽排序
//拖拽
import generatorHook from './hook/generatorHook'
generatorHook(formTableData, 'form-table-config')

const getFormTableData = () => {
  formTableData.forEach((fItem) => {
    fItem.optionDataArr = splitTheOptionArr(fItem.optionData)
  })
  return formTableData
}

defineExpose({ setFormTableData, getFormTableData })
</script>

<style scoped lang="scss"></style>
