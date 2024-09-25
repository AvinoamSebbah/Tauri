<script setup lang="ts">
import { NodeService } from '@/service/NodeService';
import { onMounted, ref } from 'vue';

const treeValue = ref(null) as any;
const selectedTreeValue = ref(null) as any;
const treeTableValue = ref(null) as any;
const selectedTreeTableValue = ref(null) as any;

onMounted(() => {
    NodeService.getTreeNodes().then((data) => (treeValue.value = data));
    NodeService.getTreeTableNodes().then((data) => (treeTableValue.value = data));
});
</script>

<template>
    <div class="card">
        <div class="font-semibold text-xl">Tree</div>
        <Tree :value="treeValue" selectionMode="checkbox" v-model:selectionKeys="selectedTreeValue"></Tree>
    </div>

    <div class="card">
        <div class="font-semibold text-xl mb-4">TreeTable</div>
        <TreeTable :value="treeTableValue" selectionMode="checkbox" v-model:selectionKeys="selectedTreeTableValue">
            <Column field="name" header="Name" :expander="true"></Column>
            <Column field="size" header="Size"></Column>
            <Column field="type" header="Type"></Column>
        </TreeTable>
    </div>
</template>
