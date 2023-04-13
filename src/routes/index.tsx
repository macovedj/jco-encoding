import { component$ } from '@builder.io/qwik';
import type { DocumentHead } from '@builder.io/qwik-city';

// import Counter from '~/components/starter/counter/counter';
import Hero from '~/components/starter/hero/hero';
import Infobox from '~/components/starter/infobox/infobox';
import Starter from '~/components/starter/next-steps/next-steps';
import { foo, $init } from "~/example-wasm/example";

export default component$(() => {
  return (
    <>
      <Hero />

      <div class="section bright">
        <div class="container center">
          <Starter />
        </div>
      </div>

      <button onClick$={async () => {
        await $init
        foo.helloWorld({a: "foo", b: "longer string by many times"})
      }}>Wasm Functions</button>
      <div class="section">
        <div class="container topics">
          <Infobox>
            <div q:slot="title" class="icon icon-cli">
              CLI Commands
            </div>
            <>
              <p>
                <code>npm run dev</code>
                <br />
                Starts the development server and watches for changes
              </p>
              <p>
                <code>npm run preview</code>
                <br />
                Creates production build and starts a server to preview it
              </p>
              <p>
                <code>npm run build</code>
                <br />
                Creates production build
              </p>
              <p>
                <code>npm run qwik add</code>
                <br />
                Runs the qwik CLI to add integrations
              </p>
            </>
          </Infobox>

          <div>
            <Infobox>
              <div q:slot="title" class="icon icon-apps">
                Example Apps
              </div>
              <p>
                Have a look at the <a href="/demo/flower">Flower App</a> or the{' '}
                <a href="/demo/todolist">Todo App</a>.
              </p>
            </Infobox>

            <Infobox>
              <div q:slot="title" class="icon icon-community">
                Community
              </div>
              <ul>
                <li>
                  <span>Questions or just want to say hi? </span>
                  <a href="https://qwik.builder.io/chat" target="_blank">
                    Chat on discord!
                  </a>
                </li>
                <li>
                  <span>Follow </span>
                  <a href="https://twitter.com/QwikDev" target="_blank">
                    @QwikDev
                  </a>
                  <span> on Twitter</span>
                </li>
                <li>
                  <span>Open issues and contribute on </span>
                  <a href="https://github.com/BuilderIO/qwik" target="_blank">
                    GitHub
                  </a>
                </li>
                <li>
                  <span>Watch </span>
                  <a href="https://qwik.builder.io/media/" target="_blank">
                    Presentations, Podcasts, Videos, etc.
                  </a>
                </li>
              </ul>
            </Infobox>
          </div>
        </div>
      </div>
    </>
  );
});

export const head: DocumentHead = {
  title: 'Welcome to Qwik',
  meta: [
    {
      name: 'description',
      content: 'Qwik site description',
    },
  ],
};
