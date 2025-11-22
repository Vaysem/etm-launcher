# ETM-launcher HTML як exe файл для <b>*.etm</b> файлів

 <img src="https://github.com/Vaysem/etm-launcher/blob/main/src-tauri/icons/favicons.ico" alt="ico" width="24" style="margin:0 0 -6px 0"/> Безпечний та сучасний HTML лаунчер для локальних файлів "file:///" який використовує власне розширення etm. По суті це аналог застарілого .hta Але з сучасними можливостями css/html/js. Та параноїдальною тендннцію безпеки, як і у всіх сучасних браузерів - відносно до локальних файлів.

---------
- Лаунчер розроблений для <u>windows 11 x64</u>. Використовує 
вбудованний <b>WebView2</b> в операційну систему windows. Через що має надзвичайно малий розмір 3,4 Мб інсталятора, і 9,8 Мб після встановлення.
---------
## Основні особливості
1. Відпрацьовує аналогічно як будь-який локальний html файл, з підключенням всіх зовнішніх ресурсів: css / html / js - при дотриманні відносного шляху [<i>src="image/1.jpg"</i>].
1. Встановлює з "свого" тега \<meta ... type="window"\> основні налаштування вікна. Приклад з усіма підтримуючими параметрами: \<meta width="310" height="460" resize="false" maximize="false" type="window"> Кожен з них - не є обов'язковим, та відповідає назві.
   * **width:** int (число) px  --ширина.
   * **height:** int (число) px --висота.
   * **resize:** bool (true/false) --дозволити/заборонити змінювати розмір віка мишкою.
   * **maximize:** bool (true/false) --дозволити/заборонити повноекранний розмір віка.

1. Встановлює в шапку WebView2 файл іконки якщо такий є, з відповідного тегу \<link rel="icon" href="favicon.ico" type="image/x-icon">. Підтримує формати <b>png</b> та <b>ico</b>.
1. Встановлює назву в шапку WebView2 з тега title - інакше назва буде братись з імені файлу або трикрапки - якщо назва сильно всрата.
1. Можливість дебажити в Edg DevTools який можна викликати за стандартною комбінацією клавіш Ctrl + Shift + I
----------
Застосунок etm-launcher без відповідного "etm" файлу - на пряму відпрацьовувати не буде. Для прикладу ви можете змінити розширення будьякого робочого html файлу та запустити його подвійним кліком як звичайну програму. Потім налаштувати файл під власні потреби.

----------

 - <b>etm-launcher</b> 22-11-2025 beta для <u>[windows 11 x64 v 0.1.0](https://github.com/Vaysem/etm-launcher/blob/main/src-tauri/target/release/bundle/msi/etm-launcher_0.1.0_x64_en-US.msi)</u> v0.1.0
-----------

## Застосовує Rust v1.93.0 і Tauri v2.9.3
<div align="center">

[Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [Rust](https://rust-lang.org/)

![Image alt](data:image/webp;base64,UklGRvwDAABXRUJQVlA4WAoAAAAYAAAAHwAAHwAAQUxQSKAAAAABkCRJkiJJ+fR6Sr3FhjGGmZmZbnYoyMzefUBETECawMaKTUrJyqmt1ZodEAaEAWFBAAEEKBEQBoQBUeJf3KDmZPwHn85Lv331nFNxPy8gRm1k7QtuVLi2412pRe3TxSJ/jvErb1+VQdXtjFnzXBwHx+rR0SnsHB2p53RHwZ8q2KU/g/WhbwrBV+eLkXl0yHoblX2trZXbVCul1NRo0gQCVlA4IFgCAADwDQCdASogACAAPjkUhUEhBgGABMDhPIAO9n6Aniv0APKL/Zn4IP2l9JKgTceB+Kuepxxf+g0hD+ifjpruv8t9gHv5fxn2S+1D56/3XuEfyP+a/5T8y+5X+s3sI/sk15rLAfsdkp4r2A/HkGJ+AsH4etq2nrzENgAA/v/lcP8gfmo3/85//u2M/MXRa3CU8//3NnBaH14e27+8UdnFi034L9AzW/k+gruO64APA75n/LU///9sMFR+qxbupwcR/3UV/m3+o6/1oJlUsv9f/5Z/ff8qLUF9GP9MQIgQ2ozApzG7cxNVpJW6BCD/+vxv/+XG/39L/4WByjiH+Hfg+jRDEk0b9EtoX3APGn/FVE+Arx7MM0SZopzmoOv8rP3bX9kl3918xAI/APaHBZAYzPifuEPf+PS/eVI/GOGqsd+gKSovfkmywEvec0u8m/8iT79D3D/9kQ/VKdOTNLh9//yuWHhB1zQiNfPmgveV71LiW4iwAvdEROks95ebwa1uwPHblH7SStNRU22DsLdsqVsO0NznFJshnGiINGKNiy1ua5+5B1MnieqZtEfy27VtUf3biJoTn/6HOaqLH4R1G5YAtO4lRBvPnm6YfUADzd77YPpnOj9+GrP/Dlev3zY+Wq/B2/yTOi3fU0mfJ9OP09h8ZBl4mAUjUBpl2GE/xYAwEYkPs+rWaYtCbfOvZ+VqlHkG3YwjMY1bqnJfaOstVJE42DreliAJGR5Ru8REiytPBXojnXXd4fFNJ3rqeSlHnpoLbxS2HgZCb8rvklRgengwasXa8gQAAABFWElG1gAAAElJKgAIAAAABgASAQMAAQAAAAEAAAAaAQUAAQAAAFYAAAAbAQUAAQAAAF4AAAAoAQMAAQAAAAIAAAAxAQIAEAAAAGYAAABphwQAAQAAAHYAAAAAAAAAYAAAAAEAAABgAAAAAQAAAFBhaW50Lk5FVCA1LjEuOQAFAACQBwAEAAAAMDIzMAGgAwABAAAAAQAAAAKgBAABAAAAIAAAAAOgBAABAAAAIAAAAAWgBAABAAAAuAAAAAAAAAACAAEAAgAEAAAAUjk4AAIABwAEAAAAMDEwMAAAAAA=)

<sub>Ⓒ 2025 etm-launcher</sub></div>