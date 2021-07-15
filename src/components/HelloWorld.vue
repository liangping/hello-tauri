<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <p>
      <a v-on:click="clickme">Click Me</a>
      <input placeholder="Input a Number" v-model="message">
      <br/>

    </p>
  </div>
</template>

<script>
// With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:
const invoke = window.__TAURI__.invoke

export default {
  name: 'HelloWorld',
  props: {
    msg: String
  },
  data: function() {
    return {
      message: ""
    }
  },
  methods:{
    clickme: function(){
      var t = new Date().toLocaleString();
      invoke('my_custom_command', {msg: t }).then((retMsg)=> this.message = retMsg)
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
