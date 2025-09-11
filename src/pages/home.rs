use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let asm_map = vec![
        ("sub_ff000", r#"sub_ff000:
    mov rax, rdi
    sub rax, 0xff000
    ret
"#),
        ("add_range", r#"add_range:
    xor rax, rax
    test rdx, rdx
    jz .done
.loop:
    mov r8, [rdi]
    add rax, r8
    add rdi, 8
    dec rdx
    jnz .loop
.done:
    ret
"#),
        ("mul_10", r#"mul_10:
    mov rax, rdi
    imul rax, 10
    ret
"#),
        ("div_2", r#"div_2:
    mov rax, rdi
    shr rax, 1
    ret
"#),
        ("and_mask", r#"and_mask:
    mov rax, rdi
    and rax, 0xff
    ret
"#),
        ("or_mask", r#"or_mask:
    mov rax, rdi
    or rax, 0xff00
    ret
"#),
        ("xor_mask", r#"xor_mask:
    mov rax, rdi
    xor rax, 0xffff
    ret
"#),
        ("shift_left", r#"shift_left:
    mov rax, rdi
    shl rax, 2
    ret
"#),
        ("shift_right", r#"shift_right:
    mov rax, rdi
    shr rax, 2
    ret
"#),
        ("negate", r#"negate:
    mov rax, rdi
    neg rax
    ret
"#),
    ];

    let selected = use_state(|| 0usize);

    let menu_items: Html = asm_map
        .iter()
        .enumerate()
        .map(|(i, (label, _))| {
            let selected = selected.clone();
            let onclick = Callback::from(move |_| selected.set(i));
            html! {
                <button onclick={onclick} class="block w-full text-left px-4 py-2 hover:bg-gray-700 border-b border-gray-800">
                    { *label }
                </button>
            }
        })
        .collect();

    let (_, code) = &asm_map[*selected];

    html! {
        <div class="min-h-screen bg-black text-white flex">
            <div class="w-48 bg-gray-900 border-r border-gray-800 overflow-y-auto">
                { menu_items }
            </div>

            <div class="flex-1 flex items-center justify-center p-6">
                <div style="width:500px; height:500px; border:2px solid white; background:#0b1220; border-radius:8px; box-shadow: 0 6px 18px rgba(0,0,0,0.6); overflow:auto;">
                    <pre style="padding:16px; margin:0; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, 'Roboto Mono', 'Courier New', monospace; font-size:13px; line-height:1.4; color:#dbeafe;">
                        <code>{ *code }</code>
                    </pre>
                </div>
            </div>
        </div>
    }
}