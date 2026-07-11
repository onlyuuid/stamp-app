<template>
    <div class="main">
        <div class="content-container">
        <div class="cards-wrapper">
            <a-row :gutter="[24, 24]" wrap justify="start">
                <a-col :span="12" v-for="item in taskList" :key="item">
                    <div class="card-container-fixed">
                        <TaskCard :taskData="item" 
                        @refresh="handleRefresh"
                        @editForm="editForm"
                        ></TaskCard>
                    </div>
                </a-col>
                <div style="flex: 1;">
                    <a-empty description="暂无任务" v-if="taskList.length == 0"/>
                </div>
            </a-row>
        </div>
        </div>
        <div class="add-box">
            <div class="icon">
                 <a-image
                    :width="50"
                    :src="paddy"
                />
            </div>
            <div class="fool-desc">
                <div>注水提示: 30分钟对应一颗水滴哦</div>
            </div>
            <div class="but">
                <a-button @click="addTask" icon="+" type="primary">创建任务</a-button>
            </div>
        </div>

        <!-- 添加任务对话框 -->
         <a-modal v-model:open="open" 
            title="添加任务" 
            cancelText="取消"
            okText="确认"
            @ok="handleOk">
            <a-form
                :model="form"
                name="basic"
                :label-col="{ span: 5 }"
                :wrapper-col="{ span: 18 }"
                autocomplete="off"
            >
                <a-form-item
                label="任务名"
                name="taskName"
                :rules="[{ required: true, message: '请输入任务名...' }]"
                >
                <a-input v-model:value="form.taskName" />
                </a-form-item>
                <a-form-item
                label="描述"
                name="description"
                :rules="[{ required: true, message: '请输入任务描述...' }]"
                >
                <a-input v-model:value="form.description" />
                </a-form-item>
            </a-form>
        </a-modal>
    </div>
</template>
<script  setup lang="ts">
import TaskCard from '../components/TaskCard.vue';
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import paddy from '../assets/image/paddy.png'
import { listTask,createTask, updateTask } from "../api/task";
import type { Task } from "../types/task";
import { useWaterStore} from '../stores/waterStore'


/**
 * 全局变量
 */
const open = ref()
const taskList = ref<Task[]>([]);
const waterStore = useWaterStore();

/** 获取今天日期 */
const getToday = () => {
    return new Date().toISOString().slice(0, 10)
}

const form = ref<Task>(
    {
       taskId:0,
       taskName: "",
       description: "",
       status:0,
       addUpNum:0,
       dayUpNum:0,
       addDayNum:0,
       createDate: getToday(),
       lastRecordDate: "",
       themeKey:""
    },
)

onMounted(() => {
    getTaskList();
})

/** 获取任务列表 */
const getTaskList = async () => {
  taskList.value = await listTask(); 
};

/** 触发刷新 */
const handleRefresh = () => {
    getTaskList();
    waterStore.refreshTodayWater();
}


/** 添加任务 */
const addTask = () => {
    open.value = true;
}

/** 编辑任务 */
const editForm = async (task:Task) => {
    open.value = true;
    form.value.taskId = task.taskId;
    form.value.taskName = task.taskName;
    form.value.description = task.description;
}


/** 提交任务 */
const handleOk = async () => {

    if(form.value.taskId != 0 ){
        // 更新数据 
        let data = await updateTask(form.value);
        if(data.taskId != null){
            reset();
            message.success('更新成功');
            open.value = false;
            getTaskList();
        }

    }else{
        // 新建数据
        let data = await createTask(form.value)
        if(data.taskId != null){
            reset();
            message.success('添加成功');
            open.value = false;
            getTaskList()
        }
    }
}





/** 重置表单 */
const reset = () => {
    form.value =  {
       taskId:0,
       taskName: "",
       description: "",
       status:0,
       addUpNum:0,
       dayUpNum:0,
       addDayNum:0,
       createDate: getToday(),
       lastRecordDate: "",
       themeKey:""
    }
}

</script>

<style lang="scss" scoped>
.main{
    width: 100%;
    height: calc(100% - 120px);
    position: relative;
    .content-container{
        flex: 1;
        overflow-y: auto;
        overflow-x: hidden;
       /* 核心：禁止文本被选中 */
        user-select: none; 
        /* 兼容旧版本浏览器的写法（可选） */
        -webkit-user-select: none; /* Safari */
        -moz-user-select: none;    /* Firefox */
        -ms-user-select: none;     /* IE10+ */
        .cards-wrapper {
            width: 100%;
            box-sizing: border-box;
        }
        .card-container-fixed {
            width: 100%;
            max-width: 620px; 
            margin: 0 auto 0 0; /* 靠左对齐 */
        }
    }
    .add-box{
        width: 65vw;
        // border: 1px solid red;
        display: grid;
        grid-template-columns: 0.2fr 4fr 0.6fr;
        height: 80px;
        border-radius: 10px;
        background-color: #fffef5;
        align-items: center;
        padding-left: 20px;
        position: fixed;
        left: 25%;
        bottom: 20px;
        /* 核心：禁止文本被选中 */
        user-select: none; 
        /* 兼容旧版本浏览器的写法（可选） */
        -webkit-user-select: none; /* Safari */
        -moz-user-select: none;    /* Firefox */
        -ms-user-select: none;     /* IE10+ */
        z-index: 10;
        .icon{
            // border: 1px solid red;
        //   width: 20px;
        //   height: 20px;
            padding: 10px;
        }
        // .fool-desc{

        // }
        .but{
            padding-right: 10px;
        }
    }
}




</style>