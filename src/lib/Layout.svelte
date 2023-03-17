<div class="h-full flex flex-col mx-2 sm:mx-10 lg:mx-40">
    <div class="flex-none mt-5 mb-2">
        <slot name="header">
            <h1>Default header</h1>
        </slot>
    </div>
    <div class="grow">
        <slot name="content">
            <p>Default content</p>
        </slot>
    </div>
    <div class="flex-none my-1">
        <slot name="footer">
            <p>Default footer</p>
        </slot>
    </div>
</div>
