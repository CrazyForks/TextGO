/**
 * 表示加载状态的响应式类
 */
export class Loading {
  private loading = $state<boolean | null>(null);

  /**
   * 将加载状态设置为`false`，然后在指定的延迟后将其设置为`true`
   *
   * @param delay - 在将加载状态设置为`true`之前等待的毫秒数
   */
  start(delay: number = 500) {
    this.loading = false;
    if (delay <= 0) {
      this.loading = true;
    } else {
      setTimeout(() => {
        if (this.loading === false) this.loading = true;
      }, delay);
    }
  }

  /**
   * 将加载状态设置为`null`，表示加载结束
   */
  end() {
    this.loading = null;
  }

  /**
   * 获取当前加载状态
   */
  get current() {
    return this.loading;
  }

  /**
   * 是否已经开始加载
   */
  get started() {
    return this.loading !== null;
  }

  /**
   * 是否已经达到延迟时间
   */
  get delayed() {
    return this.loading === true;
  }
}
