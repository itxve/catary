import { ref, onUnmounted } from "vue";
export default function () {
  const blobUrl = ref();
  function createObjectURL(blob: Blob) {
    blobUrl.value = URL.createObjectURL(blob);
    return blobUrl.value;
  }
  // 在组件卸载时释放 URL 对象
  onUnmounted(() => {
    if (blobUrl.value) {
      URL.revokeObjectURL(blobUrl.value);
      blobUrl.value = null;
    }
  });

  // 返回到模板中使用的值
  return {
    createObjectURL,
  };
}
