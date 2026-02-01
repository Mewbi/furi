import { defineStore } from 'pinia';
import { ref } from 'vue';

export enum NotificationType {
    Success = 'success',
    Warning = 'warning',
    Error = 'danger',
  }

export type Notification = {
    id: string;
    text: string;
    type: NotificationType; 
}

export const useNotifyStore = defineStore('notify', () => {
  const notifications = ref<Notification[]>([]);

  const clear = (notificationId: string): void => {
    notifications.value = notifications.value.filter((n) => n.id !== notificationId);
  };

  const notify = (text: string, type: NotificationType) => {
    const notification :Notification = { 
        id: `${Date.now()}`,
        text: text,
        type: type, 
    };
    notifications.value.push(notification);

    setTimeout(() => {
      clear(notification.id);
    }, 3_000);
  };

  return { notify, clear, notifications};
});
