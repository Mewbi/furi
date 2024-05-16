<script setup lang="ts"></script>

<template>
  <div class="w-full h-screen bg-white dark:bg-gray-800 p-8">
    <div class="flex flex-col items-center justify-center h-full">
      <img src="../assets/go-fight.png" class="w-40 h-40 mb-6" />

      <h1 class="text-2xl font-semibold text-gray-900 dark:text-white mb-6">Furi</h1>
      <div class="dark:text-white w-full max-w-md text-wrap space-y-2 pl-1 pr-1">
        <p>A <b>F</b>ast way to short an <b>URI</b>, in other words, an URL shortener.</p>
        <p>It's simple, just paste your URL and get a small one.</p>
        <p><s>And more, you can check analytics metrics of your URL.</s></p>
      </div>

      <div class="mt-12 w-full max-w-md relative mb-4">
        <div class="absolute inset-y-0 start-0 flex items-center ps-2.5 pointer-events-none">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-6 h-6 text-gray-800 dark:text-gray-400"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="m6.115 5.19.319 1.913A6 6 0 0 0 8.11 10.36L9.75 12l-.387.775c-.217.433-.132.956.21 1.298l1.348 1.348c.21.21.329.497.329.795v1.089c0 .426.24.815.622 1.006l.153.076c.433.217.956.132 1.298-.21l.723-.723a8.7 8.7 0 0 0 2.288-4.042 1.087 1.087 0 0 0-.358-1.099l-1.33-1.108c-.251-.21-.582-.299-.905-.245l-1.17.195a1.125 1.125 0 0 1-.98-.314l-.295-.295a1.125 1.125 0 0 1 0-1.591l.13-.132a1.125 1.125 0 0 1 1.3-.21l.603.302a.809.809 0 0 0 1.086-1.086L14.25 7.5l1.256-.837a4.5 4.5 0 0 0 1.528-1.732l.146-.292M6.115 5.19A9 9 0 1 0 17.18 4.64M6.115 5.19A8.965 8.965 0 0 1 12 3c1.929 0 3.716.607 5.18 1.64"
            />
          </svg>
        </div>
        <input
          type="text"
          id="input-group-1"
          class="bg-gray-100 text-gray-900 text-sm rounded-md focus:ring-blue-500 focus:border-blue-500 block w-full ps-10 p-2.5 dark:bg-gray-700 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
          placeholder="https://github.com/Mewbi"
        />
        <button
          type="submit"
          class="absolute top-0 end-0 p-2.5 h-full text-sm font-medium text-white dark:text-black bg-gray-600 rounded-e-md border hover:bg-gray-700 focus:ring-4 focus:outline-none focus:ring-gray-300 dark:bg-gray-200 dark:hover:bg-gray-300 dark:focus:ring-gray-800"
        >
          Short
        </button>
      </div>

      <div class="w-full max-w-md mt-8">
        <h2 class="text-lg text-center font-bold text-gray-900 dark:text-white mb-4">
          Your Shortened URLs
        </h2>
        <div class="space-y-4">
          <div
            class="flex items-center justify-between bg-gray-100 dark:bg-gray-700 p-3 rounded-md"
          >
            <div>
              <p class="text-sm text-gray-900 dark:text-white mb-1">short.ly/3rFg4</p>
              <p class="text-xs text-gray-500">https://github.com/Mewbi</p>
            </div>
            <button
              class="dark:text-white inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3"
            >
              Copy
            </button>
          </div>
          <div
            class="flex items-center justify-between bg-gray-100 dark:bg-gray-700 p-3 rounded-md"
          >
            <div>
              <p class="text-sm text-gray-900 dark:text-white mb-1">short.ly/7tYh1</p>
              <p class="text-xs text-gray-500">https://github.com/Mewbi</p>
            </div>
            <button
              class="dark:text-white inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3"
            >
              Copy
            </button>
          </div>
        </div>
      </div>

      <div class="flex justify-center w-full max-w-md mt-8 space-x-5">
        <a
          target="_blank"
          href="https://github.com/Mewbi/furi"
          class="hover:underline hover:text-gray-900 me-4 md:me-6 flex gap-2"
        >
          <svg
            class="w-6 h-6 text-gray-600 dark:text-gray-400"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <title>Github</title>
            <path
              fill-rule="evenodd"
              d="M10 .333A9.911 9.911 0 0 0 6.866 19.65c.5.092.678-.215.678-.477 0-.237-.01-1.017-.014-1.845-2.757.6-3.338-1.169-3.338-1.169a2.627 2.627 0 0 0-1.1-1.451c-.9-.615.07-.6.07-.6a2.084 2.084 0 0 1 1.518 1.021 2.11 2.11 0 0 0 2.884.823c.044-.503.268-.973.63-1.325-2.2-.25-4.516-1.1-4.516-4.9A3.832 3.832 0 0 1 4.7 7.068a3.56 3.56 0 0 1 .095-2.623s.832-.266 2.726 1.016a9.409 9.409 0 0 1 4.962 0c1.89-1.282 2.717-1.016 2.717-1.016.366.83.402 1.768.1 2.623a3.827 3.827 0 0 1 1.02 2.659c0 3.807-2.319 4.644-4.525 4.889a2.366 2.366 0 0 1 .673 1.834c0 1.326-.012 2.394-.012 2.72 0 .263.18.572.681.475A9.911 9.911 0 0 0 10 .333Z"
              clip-rule="evenodd"
            />
          </svg>
        </a>
        <a
          target="_blank"
          href="https://www.linkedin.com/in/felipe-fernandes-gsc/"
          class="hover:underline hover:text-gray-900 me-4 md:me-6 flex gap-2"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 16 16"
            fill="none"
            role="img"
            aria-labelledby="akli8arz4m5qzg9bckn62cdne70zuumh"
            class="w-6 h-6 octicon text-gray-600 dark:text-gray-400"
          >
            <title>LinkedIn</title>
            <g clip-path="url(#clip0_202_91845)">
              <path
                d="M14.5455 0H1.45455C0.650909 0 0 0.650909 0 1.45455V14.5455C0 15.3491 0.650909 16 1.45455 16H14.5455C15.3491 16 16 15.3491 16 14.5455V1.45455C16 0.650909 15.3491 0 14.5455 0ZM5.05746 13.0909H2.912V6.18764H5.05746V13.0909ZM3.96291 5.20073C3.27127 5.20073 2.712 4.64 2.712 3.94982C2.712 3.25964 3.272 2.69964 3.96291 2.69964C4.65236 2.69964 5.21309 3.26036 5.21309 3.94982C5.21309 4.64 4.65236 5.20073 3.96291 5.20073ZM13.0938 13.0909H10.9498V9.73382C10.9498 8.93309 10.9353 7.90327 9.83491 7.90327C8.71855 7.90327 8.54691 8.77527 8.54691 9.67564V13.0909H6.40291V6.18764H8.46109V7.13091H8.49018C8.77673 6.58836 9.47636 6.016 10.52 6.016C12.6924 6.016 13.0938 7.44582 13.0938 9.30473V13.0909V13.0909Z"
                fill="currentColor"
              ></path>
            </g>
          </svg>
        </a>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
